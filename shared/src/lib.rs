use moonlight::*;
use serde::{Deserialize, Serialize};

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum UpMsg {
    SendQuery(String),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum DownMsg {
    SearchResult(Track),
}

// ------ Track -------

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    pub format: String,
    pub track_id: String,
    pub title: String,
    pub artist: String,
}
