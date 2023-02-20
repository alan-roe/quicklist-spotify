use rspotify::{
    model::{SearchResult, SearchType},
    prelude::BaseClient,
    ClientCredsSpotify,
};
use shared::*;
use std::sync::Arc;
use zoon::{eprintln, println, *};
pub mod view;

static STORAGE_KEY: &str = "quicklist-spotify";

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
fn current_track() -> &'static Mutable<Track> {
    Mutable::new(Track {
        format: "".to_owned(),
        track_id: "".to_owned(),
        title: "".to_owned(),
        artist: "".to_owned(),
    })
}

#[static_ref]
fn new_track_title() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn new_query() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn token() -> &'static Mutable<rspotify::Token> {
    Mutable::new(rspotify::Token::default())
}

#[static_ref]
fn client() -> &'static Mutable<ClientCredsSpotify> {
    refresh_token();
    connection();
    Mutable::new(ClientCredsSpotify::from_token(token().get_cloned()))
}

pub fn refresh_token() {
    Task::start(async {
        let result = connection().send_up_msg(UpMsg::RequestToken).await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

#[static_ref]
fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| match down_msg {
        DownMsg::Token(toke) => {
            println!("DownMsg: {:?}", toke);
            token().set(toke);
        }
    })
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

// ------ ------
//   Commands
// ------ ------

fn add_track(track: Option<&Track>) {
    let mut new_query = new_query().lock_mut();
    if !search_results().lock_ref().is_empty() {
{            tracks()
            .lock_mut()
            .push_cloned(Arc::new(track.unwrap_or(search_results().lock_ref().first().unwrap()).clone()));}
        save_tracks();
        search_results().lock_mut().clear();
        new_query.clear();
    }
    
}

pub fn load_tracks() {
    if let Some(Ok(tracks)) = local_storage().get(STORAGE_KEY) {
        replace_tracks(tracks);
        println!("Tracks loaded");
    }
}

fn search() {
    Task::start(async {
        let query = new_query().get_cloned();
        let query = query.trim();
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
                    //                        format!("Title: {} | Artist: {} | Track ID: {}", track.name, track.artists[0].name, track.id.as_ref().unwrap())
                    println!("Title: {} | Artist: {}", &track.name, track.artists[0].name);
        
                    results.push_cloned(Arc::new(Track {
                        format: format!("{} - {}", &track.name, &track.artists[0].name),
                        track_id: track.id.unwrap().to_string(),
                        title: track.name.clone(),
                        artist: track.artists[0].name.clone(),
                    }));
                }
            }
        }
    })
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

fn set_new_query(query: String) {
    new_query().set(query)
}

fn remove_track(id: &str) {
    tracks()
        .lock_mut()
        .retain(|track| track.track_id.as_str() != id);
    save_tracks();
}
