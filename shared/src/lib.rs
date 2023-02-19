use moonlight::*;

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    RequestToken,
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    Token(rspotify::Token),
}

// ------ Track -------

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "serde")]
pub struct Track {
    pub format: String,
    pub track_id: String,
    pub title: String,
    pub artist: String,
}
