use crate::app;
use zoon::{println, *};

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|_: Option<Route>| async {
        println!("{}", routing::url());
        let url = routing::url();

        if url.contains("code") {
            app::response_url().set(url);
            app::authorize_client();
            router().replace(routing::origin());
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
