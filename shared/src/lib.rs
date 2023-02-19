use moonlight::*;
use serde::{Deserialize, Serialize};

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum UpMsg {
    RequestToken,
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum DownMsg {
    Token(rspotify::Token),
}

// ------ Track -------

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    pub format: String,
    pub track_id: String,
    pub title: String,
    pub artist: String,
}
