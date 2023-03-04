mod app;
mod elements;
mod router;
use zoon::*;

fn main() {
    app::load_tracks();
    app::connection();
    app::token();
    router::router();
    start_app("app", app::view::root);
}
