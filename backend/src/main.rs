use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use moon::*;
use rspotify::{self, prelude::BaseClient, ClientCredsSpotify, Credentials, Token};
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

#[async_recursion]
async fn request_token() -> anyhow::Result<Token> {
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
    println!("Retrieving token...");
    let token = client.get_token();
    let token = token.lock().await.unwrap().clone();

    if let Some(token) = token {
        println!("Token retrieved!");
        if token.is_expired() {
            println!("Token expired, refreshing...");
            client.refresh_token().await?;

            request_token().await
        } else {
            println!("Valid token.");
            Ok(token)
        }
    } else {
        Err(anyhow::anyhow!("Couldn't retrieve token from client"))
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

    let down_msg = match up_msg {
        UpMsg::RequestToken => DownMsg::Token(request_token().await.unwrap()),
    };

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        session.send_down_msg(&down_msg, cor_id).await;
    } else {
        println!("Failed to get session {session_id}");
    }
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
