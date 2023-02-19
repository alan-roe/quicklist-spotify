mod app;
use zoon::*;

fn main() {
    app::load_tracks();
    start_app("app", app::view::root);
}
