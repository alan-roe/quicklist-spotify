use super::*;
use crate::elements::{Search, Input};

pub fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::fill().min_screen())
        .s(Font::new()
            .size(14)
            .color(hsluv!(0, 0, 5.1))
            .weight(FontWeight::Light)
            .family([FontFamily::new("Inter"), FontFamily::SansSerif]))
        .s(Background::new().color(hsluv!(0, 0, 96.5)))
        .item(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::fill().min(230).max(1024))
        .s(Align::new().center_x())
        .item(header())
        .item(
            Column::new()
                .s(Width::fill())
                .s(Gap::both(65))
                .item(panels())
                .item(footer()),
        )
}

fn header() -> impl Element {
    El::with_tag(Tag::Header)
        .s(Padding::new().top(10))
        .s(Align::new().center_x())
        .s(Height::exact(130))
        .s(Font::new()
            .size(48)
            .color(named_color::GREEN_7)
            .weight(FontWeight::Heavy))
        .child(El::with_tag(Tag::H1).child("QuickList for Spotify"))
}

fn panels() -> impl Element {
    Row::new()
        .item(search_results_panel())
        .item(playlist_panel())
}

// ------ Search ------

fn search_results_panel() -> impl Element {
    let (focus, focus_signal) = Mutable::new_and_signal(false);
    Column::with_tag(Tag::Section)
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Align::new().top())
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .item(search_track(focus_signal))
        .item_signal(super::results_exist().map_true(move || search_results(focus.clone())))
}

fn search_result(track: Arc<Track>, input_focus: Mutable<bool>) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Gap::both(5))
        .s(Font::new().size(24))
        .item(search_info(track.clone()))
        .id(&track.track_id)
        .on_click(move || {
            add_track(Some(&track));
            input_focus.set(true);
        })
}

fn search_results(input_focus: Mutable<bool>) -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        .s(Background::new().color(hsluv!(0, 0, 93.7)))
        .s(Gap::both(1))
        .items_signal_vec(
            super::search_results()
                .signal_vec_cloned()
                .map(move |track| search_result(track, input_focus.clone())),
        )
}

fn search_track(focus: impl Signal<Item = bool> + Unpin + 'static) -> impl Element {    
    Search::new()
        .focus_signal(focus)
        .on_focus(super::start_search_timer)
        .on_change(|new_query| {
            super::set_new_query(new_query);
            super::start_search_timer();
        })
        .placeholder("Start typing a song title/artist")
        .on_key_down_event(|event| {
            event.if_key(Key::Enter, || super::add_track(None));
            event.if_key(Key::Other(" ".to_string()), super::search);
        })
        .value_signal(super::new_query().signal_cloned())
}

fn search_info(track: Arc<Track>) -> impl Element {
    Label::new()
        .s(Width::fill())
        .s(Font::new().color(hsluv!(0, 0, 32.7)).size(24))
        .s(Padding::all(15).right(60))
        .s(Clip::x())
        .for_input(track.track_id.clone())
        .label(track.format.clone())
}

// ------ Playlist ------

fn playlist_panel() -> impl Element {    
    Column::with_tag(Tag::Section)
        // .s(Shadows::new([
        //     Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
        //     Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        // ]))
        // .s(Width::fill())
        // .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Align::new().top())
        .s(AlignContent::new().top())
    
        .item(playlist_name())
        .item_signal(super::tracks_exist().map_true(tracks))
        .item_signal(super::tracks_exist().map_true(panel_footer))
}

fn playlist_name() -> impl Element {
    Row::new()
    //  .s(Padding::new().right(5))
    //  .s(Gap::both(5))
    .s(Align::new().top())
    .s(AlignContent::new().top())
    .item(playlist_name_input())
    .item_signal(super::auth_token_expired().map_bool(|| login_button().left_either(), || login_button().right_either()))
    
}

fn playlist_create_button(
) -> impl Element {
    crate::elements::Button::new()
        // .s(Background::new().color_signal(
        //     hovered_signal.map_bool(|| hsluv!(125, 100, 60), || hsluv!(125, 100, 50)),
        // ))
        // .s(Font::new()
        //     .color(hsluv!(0, 0, 5.1))
        //     .weight(FontWeight::Bold))
        // .s(Padding::new().x(20).y(10))
        // .s(RoundedCorners::all(4))
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

fn login_button(
) -> impl Element {
    crate::elements::Button::new()
        // .s(Background::new().color_signal(
        //     hovered_signal.map_bool(|| hsluv!(125, 100, 60), || hsluv!(125, 100, 50)),
        // ))
        // .s(Font::new()
        //     .color(hsluv!(0, 0, 5.1))
        //     .weight(FontWeight::Bold))
        // .s(Padding::new().x(20).y(10))
        // .s(RoundedCorners::all(4))
        .size("sm")
        .on_press(super::login)
        .label("Log in")
}

fn playlist_name_input() -> impl Element {
    let text_signal = super::playlist_name().signal_cloned();
    Input::new()
        // .s(Padding::all(15).y(19).right(60))
        // .s(Font::new().size(24).color(hsluv!(0, 0, 32.7)))
        // .s(Background::new().color(hsluv!(0, 0, 0, 0.3)))
        // .s(Borders::all_signal(focus_signal.map_bool(
        //     || Border::new().color(hsluv!(0, 0, 63.2)),
        //     || Border::new().color(hsluv!(0, 0, 91.03)),
        // )))
        // .s(Shadows::new([Shadow::new()
        //     .inner()
        //     .y(-2)
        //     .blur(1)
        //     .color(hsluv!(0, 0, 0, 3))]))
        // .s(Font::new().color(hsluv!(0, 0, 32.7)))
        // .label_hidden("playlist name")
        .s(Align::new().top())
        .s(AlignContent::new().top())
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
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Gap::both(5))
        .s(Font::new().size(24))
        .item(track_info(track))
}

fn tracks() -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        .s(Background::new().color(hsluv!(0, 0, 93.7)))
        .s(Gap::both(1))
        .items_signal_vec(super::tracks().signal_vec_cloned().map(track))
}

fn track_info(track: Arc<Track>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Label::new()
        .s(Width::fill())
        .s(Font::new().color(hsluv!(0, 0, 32.7)).size(24))
        .s(Padding::all(15).right(60))
        .s(Clip::x())
        .for_input(track.track_id.clone())
        .label(track.format.clone())
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
    let item_container = || El::new().s(Width::fill());
    Row::with_tag(Tag::Footer)
        .s(Padding::new().x(15).y(8))
        .s(Font::new().color(hsluv!(0, 0, 50)))
        .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        .s(Shadows::new([
            Shadow::new().y(1).blur(1).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(8).spread(-3).color(hsluv!(0, 0, 96.9)),
            Shadow::new()
                .y(9)
                .blur(1)
                .spread(-3)
                .color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(16).spread(-6).color(hsluv!(0, 0, 96.9)),
            Shadow::new()
                .y(17)
                .blur(2)
                .spread(-6)
                .color(hsluv!(0, 0, 0, 20)),
        ]))
        .item(item_container().child(track_count()))
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
    Column::with_tag(Tag::Footer)
        .s(Gap::both(9))
        .s(Font::new().size(10).color(hsluv!(0, 0, 77.3)).center())
        .item(
            Paragraph::new()
                .content("Created by ")
                .content(author_link()),
        )
}
