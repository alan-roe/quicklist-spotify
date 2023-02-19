mod app;
use zoon::*;

fn main() {
    app::load_tracks();
    app::init_client();
    start_app("app", app::view::root);
}
