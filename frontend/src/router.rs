use crate::app;
use zoon::{println, *};

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|_: Option<Route>| async {
        let url = routing::url();

        if url.contains("code") {
            println!("{}", &url);
            app::response_url().set(url);
            app::authorize_client();
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("?code", response_code)]
    Code { response_code: String },

    #[route()]
    Root,
}
