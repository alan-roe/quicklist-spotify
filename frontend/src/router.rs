use crate::app;
use zoon::*;

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|_: Option<Route>| async {
        let url = routing::url();

        // we got back our auth code, init our client
        if url.contains("code") {
            app::response_url().set(url);
            app::authorize_client();
            return router().replace("/");
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route()]
    Root,
}
