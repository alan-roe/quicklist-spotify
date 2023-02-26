use super::*;
use crate::elements::{Column, Grid, Input, Row, Search, Tile};

pub fn root() -> impl Element {
    Grid::new()
        .child(Row::new().child(header()))
        .child(content())
        .child(footer())
}

fn content() -> impl Element {
    Row::new().child(panels())
}

fn header() -> impl Element {
    El::with_tag(Tag::Header)
        .s(Padding::new().y(10))
        .s(Align::new().center_x())
        //.s(Height::exact(130))
        .s(Font::new()
            .size(48)
            .color(named_color::GREEN_7)
            .weight(FontWeight::Heavy))
        .child(El::with_tag(Tag::H1).child("QuickList for Spotify"))
}

fn panels() -> impl Element {
    Column::new()
        .child(
            RawHtmlEl::new("div")
                .class("bx--row")
                .child(search_results_panel()),
        )
        .child(
            RawHtmlEl::new("div")
                .class("bx--row")
                .child(playlist_panel()),
        )
}

// ------ Search ------

fn search_results_panel() -> impl Element {
    let (focus, focus_signal) = Mutable::new_and_signal(false);
    Column::new()
        .child(search_track(focus_signal))
        .child_signal(super::results_exist().map_true(move || search_results(focus.clone())))
}

fn search_result(track: Arc<Track>, input_focus: Mutable<bool>) -> impl Element {
    Row::new().child(search_info(track, input_focus))
}

fn search_results(input_focus: Mutable<bool>) -> impl Element {
    Column::new().children_signal_vec(
        super::search_results()
            .signal_vec_cloned()
            .map(move |track| search_result(track, input_focus.clone())),
    )
}

fn search_track(focus: impl Signal<Item = bool> + Unpin + 'static) -> impl Element {
    Search::new()
        .focus(true)
        .focus_signal(focus)
        .on_focus(super::start_search_timer)
        .on_change(|new_query| {
            super::set_new_query(new_query);
            super::start_search_timer();
        })
        .placeholder("Start typing a song title/artist")
        .label("Search for track")
        .on_key_down_event(|event| {
            event.if_key(Key::Enter, || super::add_track(None));
            event.if_key(Key::Other(" ".to_string()), super::search);
        })
        .value_signal(super::new_query().signal_cloned())
        .size("lg")
}

fn search_info(track: Arc<Track>, input_focus: Mutable<bool>) -> impl Element {
    Tile::new()
        .s(Width::fill())
        .s(Padding::all(15).right(60))
        .child(track.format.clone())
        .on_click(move || {
            add_track(Some(&track));
            input_focus.set(true);
        })
}

// ------ Playlist ------

fn playlist_panel() -> impl Element {
    Column::with_tag(Tag::Section)
        .child(playlist_name())
        .child_signal(super::tracks_exist().map_true(tracks))
        .child_signal(super::tracks_exist().map_true(panel_footer))
}

fn playlist_name() -> impl Element {
    Row::new()
        .s(Width::fill())
        .child(Column::new().child(playlist_name_input()))
        .child(
            Column::new()
                .child_signal(super::auth_token_expired().map_bool(
                    || login_button().left_either(),
                    || playlist_create_button().right_either(),
                ))
                .update_raw_el(|x| x.style("align-self", "flex-end")),
        )
}

fn playlist_create_button() -> impl Element {
    crate::elements::Button::new()
        .on_press(|| {
            if !super::playlist_created().get() {
                super::create_playlist();
            }
        })
        .label_signal(
            super::playlist_created()
                .signal()
                .map_bool(|| "Created", || "Create"),
        )
}

fn login_button() -> impl Element {
    crate::elements::Button::new()
        .size("md")
        .on_press(super::login)
        .label("Log in")
}

fn playlist_name_input() -> impl Element {
    let text_signal = super::playlist_name().signal_cloned();
    Input::new()
        .placeholder("Playlist Name")
        .s(Align::new().top())
        .s(AlignContent::new().top())
        .size("md")
        .label("Playlist Name")
        .update_raw_el(|x| x.style("white-space", "normal"))
        .on_blur(super::store_playlist_name)
        .on_change(move |text| super::playlist_name().set_neq(text))
        .on_key_down_event(|event| match event.key() {
            Key::Escape => super::reload_playlist_name(),
            Key::Enter => super::store_playlist_name(),
            _ => (),
        })
        .value_signal(text_signal)
}

fn track(track: Arc<Track>) -> impl Element {
    zoon::Row::new()
        .s(Width::fill())
        //.s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Gap::both(5))
        //.s(Font::new().size(24))
        .item(track_info(track))
}

fn tracks() -> impl Element {
    zoon::Column::new()
        // .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        // .s(Background::new().color(hsluv!(0, 0, 93.7)))
        .s(Gap::both(1))
        .items_signal_vec(super::tracks().signal_vec_cloned().map(track))
}

fn track_info(track: Arc<Track>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Tile::new()
        .s(Width::fill())
        .s(Padding::all(15).right(60))
        .child(track.format.clone())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .element_on_right_signal(hovered_signal.map_true(move || remove_track_button(&track)))
}

fn remove_track_button(todo: &Track) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = todo.track_id.clone();
    Button::new()
        .s(Width::exact(40))
        .s(Height::exact(40))
        .s(Transform::new().move_left(50).move_down(14))
        .s(Font::new().size(30).center().color_signal(
            hovered_signal.map_bool(|| hsluv!(10.5, 37.7, 48.8), || hsluv!(12.2, 34.7, 68.2)),
        ))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || super::remove_track(&id))
        .label("×")
}

fn panel_footer() -> impl Element {
    Row::with_tag(Tag::Footer)
        .s(Font::new().color(hsluv!(0, 0, 50)))
        .s(Padding::new().x(15))
        .child(track_count())
}

fn track_count() -> impl Element {
    Text::with_signal(
        super::track_count()
            .map(|count| format!("{} track{}", count, if count == 1 { "" } else { "s" })),
    )
}

fn author_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new().line(FontLine::new().underline_signal(hovered_signal)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Alan Roe")
        .to("https://github.com/alan-roe")
        .new_tab(NewTab::new())
}

fn footer() -> impl Element {
    Row::with_tag(Tag::Footer)
        .s(Gap::both(9))
        .s(Padding::new().x(15).y(8))
        .s(Font::new().size(10).color(hsluv!(0, 0, 77.3)).center())
        .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        .child(
            Paragraph::new()
                .content("Created by ")
                .content(author_link()),
        )
}
