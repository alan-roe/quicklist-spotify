use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use moon::*;
use rspotify::{self, prelude::BaseClient, ClientCredsSpotify, Credentials, Token};
use shared::{
    rspotify::{prelude::OAuthClient, scopes, AuthCodeSpotify, OAuth},
    *,
};

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

async fn request_auth_token(data: AuthResponseData) -> anyhow::Result<Token> {
    let creds = Credentials {
        id: "***REMOVED***".to_owned(),
        secret: Some("***REMOVED***".to_owned()),
    };
    let oauth = OAuth {
        redirect_uri: "http://192.168.1.2:8080".to_string(),
        scopes: scopes!("playlist-modify-private"),
        state: data.state,
        ..Default::default()
    };

    let auth_client = AuthCodeSpotify::new(creds, oauth);
    let response_url = data.response_url;

    if let Some(code) = auth_client.parse_response_code(&response_url) {
        println!("Successful response code parse, requesting token");
        if auth_client.request_token(&code).await.is_ok() {
            println!("Successful token request!");
        };
    } else {
        println!("failed to parse response code: {}", &response_url);
    }
    Ok(auth_client
        .get_token()
        .lock()
        .await
        .unwrap()
        .clone()
        .unwrap())
}

async fn request_auth_data() -> anyhow::Result<AuthData> {
    let oauth = OAuth {
        redirect_uri: "http://192.168.1.2:8080".to_string(),
        scopes: scopes!("playlist-modify-private", "playlist-read-private"),
        ..Default::default()
    };

    let creds = Credentials {
        id: "***REMOVED***".to_owned(),
        secret: Some("***REMOVED***".to_owned()),
    };

    let spotify = AuthCodeSpotify::new(creds, oauth.clone());
    Ok(AuthData {
        url: spotify.get_authorize_url(false)?,
        state: oauth.state,
    })
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
        UpMsg::RequestAuthData => DownMsg::AuthData(request_auth_data().await.unwrap()),
        UpMsg::RequestAuthToken(data) => {
            DownMsg::AuthToken(request_auth_token(data).await.unwrap())
        }
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
