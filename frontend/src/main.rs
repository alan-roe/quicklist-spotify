mod app;
mod router;
use zoon::*;

fn main() {
    app::load_tracks();
    router::router();
    start_app("app", app::view::root);
    app::refresh_token();
}
