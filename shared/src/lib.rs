use moonlight::*;
pub use rspotify;

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    RequestToken,
    RequestAuthData,
    RequestAuthToken(AuthResponseData),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub struct AuthData {
    pub url: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub struct AuthResponseData {
    pub response_url: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    Token(rspotify::Token),
    AuthData(AuthData),
    AuthToken(rspotify::Token),
}

// ------ Track -------

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "serde")]
pub struct Track {
    pub format: String,
    pub track_id: String,
    pub title: String,
    pub artist: String,
    pub duration_sec: i64,
}
