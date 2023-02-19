use rspotify::{ClientCredsSpotify, model::{SearchResult, SearchType}, prelude::BaseClient};
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
    Mutable::new(ClientCredsSpotify::from_token(token().get_cloned()))
}

fn request_token() {
    Task::start(async {
        let result = connection()
            .send_up_msg(UpMsg::RequestToken)
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

pub fn init_client() {
    request_token();
    connection();
}

#[static_ref]
fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::Token(toke), _| {
        println!("DownMsg: {:?}", toke);
        token().set(toke);
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

// ------ ------
//   Commands
// ------ ------

fn add_track() {
    let mut new_query = new_query().lock_mut();
    let current_track = current_track().lock_ref();
    tracks()
        .lock_mut()
        .push_cloned(Arc::new(current_track.clone()));
    save_tracks();
    new_query.clear();
}

pub fn load_tracks() {
    if let Some(Ok(tracks)) = local_storage().get(STORAGE_KEY) {
        replace_tracks(tracks);
        println!("Tracks loaded");
    }
}


fn search() {

    //if query.is_empty() {
    //
    //}
    Task::start(async {
    
    let query = new_query().get_cloned();
    let query = query.trim();
        if let Ok(search_result) = client().lock_ref()
        .search(query, SearchType::Track, None, None, Some(1), None)
        .await
    {
        use SearchResult::*;

        if let Tracks(track) = search_result {
            if let Some(track) = track.items.first() {
                //                        format!("Title: {} | Artist: {} | Track ID: {}", track.name, track.artists[0].name, track.id.as_ref().unwrap())

                let track = track.clone();
                println!("Title: {} | Artist: {}", &track.name, track.artists[0].name);
                current_track().set( Track {
                    format: format!("{} - {}", &track.name, &track.artists[0].name),
                    track_id: track.id.unwrap().to_string(),
                    title: track.name.clone(),
                    artist: track.artists[0].name.clone(),
                });
            }
        }
    }
    println!("lol");
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
