use async_once_cell::OnceCell;
use moon::*;
use rspotify::{
    self,
    model::{SearchResult, SearchType},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};
use shared::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("QuickList for Spotify")
        .append_to_head(include_str!("../favicon.html")) // realfavicongenerator.net
        .append_to_head(
            "
        <link rel=\"preconnect\" href=\"https://rsms.me/\">
        <link rel=\"stylesheet\" href=\"https://rsms.me/inter/inter.css\">
        <style>
            :root { font-family: 'Inter', sans-serif; }
            @supports (font-variation-settings: normal) {
             :root { font-family: 'Inter var', sans-serif; }
            }
        </style>",
        )
}

static CLIENT: OnceCell<ClientCredsSpotify> = OnceCell::new();

async fn search(query: &str) -> Track {
    let query = query.trim();

    let client = CLIENT
        .get_or_init(async {
            let creds = Credentials {
                id: "***REMOVED***".to_owned(),
                secret: Some("***REMOVED***".to_owned()),
            };
            println!("Creds\nid: {}\nsecret: {:?}", &creds.id, &creds.secret);

            let client = ClientCredsSpotify::new(creds);
            client.request_token().await.unwrap();
            client
        })
        .await;

    //if query.is_empty() {
    //
    //}
    if let Ok(search_result) = client
        .search(query, SearchType::Track, None, None, Some(1), None)
        .await
    {
        use SearchResult::*;

        if let Tracks(track) = search_result {
            if let Some(track) = track.items.first() {
                //                        format!("Title: {} | Artist: {} | Track ID: {}", track.name, track.artists[0].name, track.id.as_ref().unwrap())

                let track = track.clone();
                println!("Title: {} | Artist: {}", &track.name, track.artists[0].name);
                return Track {
                    format: format!("{} - {}", &track.name, &track.artists[0].name),
                    track_id: track.id.unwrap().to_string(),
                    title: track.name.clone(),
                    artist: track.artists[0].name.clone(),
                };
            }
        }
    }
    println!("lol");
    Track {
        format: format!("{} - {}", &query, &query),
        track_id: query.to_string(),
        title: query.to_string(),
        artist: Default::default(),
    }
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);

    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        ..
    } = req;
    let UpMsg::SendQuery(query) = up_msg;

    sessions::by_session_id()
        .get(session_id)
        .unwrap()
        .send_down_msg(&DownMsg::SearchResult(search(&query).await), cor_id)
        .await;
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
