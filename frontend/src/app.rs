use crate::router::router;
use rspotify::{
    model::{SearchResult, SearchType},
    prelude::BaseClient,
    ClientCredsSpotify,
};
use shared::{
    rspotify::{
        model::TrackId,
        prelude::{OAuthClient, PlayableId},
        AuthCodeSpotify, Token,
    },
    *,
};
use std::sync::Arc;
use zoon::{eprintln, println, web_storage::Result, *};
pub mod view;

static STORAGE_KEY: &str = "quicklist-spotify";

// ------ Local Storage ------

fn store_local<T: Serialize + ?Sized>(key: &str, val: &T) {
    let key = STORAGE_KEY.to_owned() + "-" + key;
    println!("Storing local: {key}");
    if let Err(e) = local_storage().insert(&key, val) {
        eprintln!("Saving {key} to local storage failed: {e}");
    } else {
        println!("Saved {key} to local storage");
    }
}

fn retrieve_local<T: DeserializeOwned>(key: &str) -> Result<Mutable<T>> {
    let key = STORAGE_KEY.to_owned() + "-" + key;
    match local_storage().get(&key) {
        Some(Ok(val)) => {
            println!("{key} loaded");
            Ok(Mutable::new(val))
        }
        Some(Err(e)) => Err(e),
        None => Err(web_storage::Error::StorageNotFoundError),
    }
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn tracks() -> &'static MutableVec<Arc<Track>> {
    MutableVec::new()
}

#[static_ref]
fn search_results() -> &'static MutableVec<Arc<Track>> {
    MutableVec::new()
}

#[static_ref]
fn query() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn playlist_name() -> &'static Mutable<String> {
    retrieve_local("playlist-name").unwrap_or_default()
}

fn store_playlist_name() {
    store_local("playlist-name", playlist_name())
}

fn reload_playlist_name() {
    if let Ok(name) = retrieve_local("playlist-name") {
        playlist_name().set(name.get_cloned());
    }
}

#[static_ref]
pub fn selected_track() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn playlist_duration() -> &'static Mutable<i64> {
    // Calculate duration from tracks
    Mutable::new(
        tracks()
            .lock_ref()
            .iter()
            .map(|x| x.duration_sec)
            .reduce(|x, acc| x + acc)
            .unwrap_or_default(),
    )
}

fn playlist_duration_format() -> String {
    let secs = playlist_duration().get();
    let min = format!("{} min", secs / 60 % 60);
    let duration = if secs < 3_600 {
        (min, format!("{} sec", secs % 60))
    } else {
        (format!("{} hr", secs / 60 / 60 % 60), min)
    };

    duration.0 + " " + &duration.1
}

#[static_ref]
pub fn token() -> &'static Mutable<Token> {
    let token: Mutable<Token> = retrieve_local("token").unwrap_or_default();

    if token.get_cloned().is_expired() {
        println!("Token expired, refreshing");
        refresh_token()
    }

    token
}

fn refresh_token() {
    send_up_msg(UpMsg::RequestToken);
}

#[static_ref]
fn auth_token() -> &'static Mutable<Token> {
    retrieve_local("auth-token").unwrap_or_default()
}

#[static_ref]
fn client() -> &'static Mutable<ClientCredsSpotify> {
    Mutable::new(ClientCredsSpotify::from_token(token().get_cloned()))
    
}

#[static_ref]
fn auth_client() -> &'static Mutable<AuthCodeSpotify> {
    Mutable::new(AuthCodeSpotify::from_token(auth_token().get_cloned()))
}

#[static_ref]
fn auth_state() -> &'static Mutable<String> {
    retrieve_local("state").unwrap_or_default()
}

#[static_ref]
pub fn response_url() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| match down_msg {
        DownMsg::Token(toke) => {
            println!("Retrieved token: {:?}", toke);
            token().set(toke);
            client().set(ClientCredsSpotify::from_token(token().get_cloned()));
            store_local("token", token());
        }
        DownMsg::AuthToken(token) => {
            println!("Retrieved auth token: {:?}", token);
            auth_token().set(token);
            auth_client().set(AuthCodeSpotify::from_token(auth_token().get_cloned()));
            store_local("auth-token", auth_token());
        }
        DownMsg::AuthData(data) => {
            println!("Retrieved auth data: {:?}", &data);
            auth_state().set(data.state);
            store_local("state", auth_state());
            router().go(data.url);
        }
    })
}

#[static_ref]
fn playlist_created() -> &'static Mutable<bool> {
    Mutable::new(false)
}

// -- search timer --

#[static_ref]
fn search_timer() -> &'static Mutable<Option<Timer>> {
    Mutable::default()
}

fn start_search_timer() {
    search_timer().set(Some(Timer::new(500, || {
        search();
        stop_search_timer();
    })));
}

fn stop_search_timer() {
    search_timer().take();
}

// ------ ------
//   Signals
// ------ ------

fn track_count() -> impl Signal<Item = usize> {
    tracks().signal_vec_cloned().len()
}

fn tracks_exist() -> impl Signal<Item = bool> {
    track_count().map(|count| count != 0).dedupe()
}

fn results_count() -> impl Signal<Item = usize> {
    search_results().signal_vec_cloned().len()
}

fn results_exist() -> impl Signal<Item = bool> {
    results_count().map(|count| count != 0).dedupe()
}

fn auth_token_expired() -> impl Signal<Item = bool> {
    auth_token().signal_ref(|t| t.is_expired())
}

// ------ ------
//   Commands
// ------ ------

fn send_up_msg(msg: UpMsg) {
    Task::start(async {
        if let Err(error) = connection().send_up_msg(msg).await {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn create_playlist() {
    Task::start(async {
        let client = auth_client().lock_ref();
        let user_id = client.current_user().await.unwrap().id;

        if let Ok(r) = client
            .user_playlist_create(
                user_id,
                &playlist_name().get_cloned(),
                Some(false),
                Some(false),
                Some("Playlist created by QuickList"),
            )
            .await
        {
            if let Ok(r) = client
                .playlist_add_items(
                    r.id.clone(),
                    tracks()
                        .lock_ref()
                        .iter()
                        .map(|t| PlayableId::Track(TrackId::from_uri(&t.track_id).unwrap())),
                    None,
                )
                .await
            {
                playlist_created().set(true);
                println!("Create playlist success!\n{:?}", r);
            };
        } else {
            println!("Failed to create playlist :(");
        }
    })
}

fn add_track(track: Option<&Track>) {
    let mut query = query().lock_mut();
    if !search_results().lock_ref().is_empty() {
        {
            let track = if let Some(track) = track {
                track.to_owned()
            } else {
                search_results()
                    .lock_ref()
                    .iter()
                    .find(|x| x.track_id.eq(&selected_track().get_cloned()))
                    .unwrap()
                    .as_ref()
                    .clone()
            };
            playlist_duration().update(|x| x + track.duration_sec);
            tracks().lock_mut().push_cloned(Arc::new(track));
        }
        save_tracks();
        search_results().lock_mut().clear();
        query.clear();
    }
}

pub fn load_tracks() {
    if let Some(Ok(tracks)) = local_storage().get(STORAGE_KEY) {
        replace_tracks(tracks);
        println!("Tracks loaded");
    }
}

fn save_tracks() {
    if let Err(error) = local_storage().insert(STORAGE_KEY, tracks()) {
        eprintln!("Saving tracks failed: {:?}", error);
    }
}

fn replace_tracks(new_tracks: Vec<Arc<Track>>) {
    tracks().update_mut(|tracks| {
        tracks.clear();
        tracks.extend(new_tracks);
    });
}

fn remove_track(id: &str) {
    tracks().lock_mut().retain(|track| {
        if track.track_id.as_str() != id {
            true
        } else {
            playlist_duration().update(|x| x - track.duration_sec);
            false
        }
    });
    save_tracks();
}

fn next_track() {
    selected_track().update_mut(|x| {
        let tracks_ref = search_results().lock_ref();
        let mut tracks_iter = tracks_ref.iter();
        while let Some(track) = tracks_iter.next() {
            if track.track_id == *x {
                if let Some(next) = tracks_iter.next() {
                    *x = next.track_id.clone();
                } else {
                    let tracks = search_results().lock_ref();
                    *x = tracks.first().unwrap().track_id.clone();
                }
                break;
            }
        }
    });
}

fn prev_track() {
    selected_track().update_mut(|x| {
        let tracks_ref = search_results().lock_ref();
        let mut tracks_iter = tracks_ref.iter().rev();
        while let Some(track) = tracks_iter.next() {
            if track.track_id == *x {
                if let Some(next) = tracks_iter.next() {
                    *x = next.track_id.clone();
                } else {
                    let tracks = search_results().lock_ref();
                    *x = tracks.last().unwrap().track_id.clone();
                }
                break;
            }
        }
    });
}

fn search() {
    Task::start(async {
        let query = query().get_cloned();
        let query = query.trim();
        if query.is_empty() {
            search_results().lock_mut().clear();
            return;
        }
        if let Ok(search_result) = client()
            .lock_ref()
            .search(query, SearchType::Track, None, None, Some(5), None)
            .await
        {
            use SearchResult::*;

            if let Tracks(tracks) = search_result {
                let mut results = search_results().lock_mut();
                results.clear();
                for track in tracks.items.into_iter() {
                    results.push_cloned(Arc::new(Track {
                        format: format!("{} - {}", &track.name, &track.artists[0].name),
                        track_id: track.id.unwrap().to_string(),
                        title: track.name.clone(),
                        artist: track.artists[0].name.clone(),
                        duration_sec: track.duration.num_seconds(),
                    }));
                }
                if let Some(track) = results.first() {
                    selected_track().set(track.track_id.clone());
                }
            }
        }
    })
}

fn auth_data() -> AuthResponseData {
    AuthResponseData {
        response_url: response_url().get_cloned(),
        state: auth_state().get_cloned(),
    }
}

pub fn authorize_client() {
    send_up_msg(UpMsg::RequestAuthToken(auth_data()));
}

fn login() {
    // begin the auth process, this will send us to the spotify auth page if we receive the auth data from backend
    send_up_msg(UpMsg::RequestAuthData);
}
