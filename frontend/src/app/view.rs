use super::*;

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
    Column::with_tag(Tag::Section)
    .s(Shadows::new([
        Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
        Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
    ]))
    .s(Width::fill())
    .s(Background::new().color(hsluv!(0, 0, 100)))
    .item(search_track())
    .item_signal(super::results_exist().map_true(search_results))
}

fn search_result(track: Arc<Track>) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Gap::both(5))
        .s(Font::new().size(24))
        .item(search_info(track))
}

fn search_results() -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(hsluv!(0, 0, 91.3))))
        .s(Background::new().color(hsluv!(0, 0, 93.7)))
        .s(Gap::both(1))
        .items_signal_vec(super::search_results().signal_vec_cloned().map(search_result))
}

fn search_track() -> impl Element {
    TextInput::new()
        .s(Padding::all(15).y(19).right(60))
        .s(Font::new().size(24).color(hsluv!(0, 0, 32.7)))
        .s(Background::new().color(hsluv!(0, 0, 0, 0.3)))
        .s(Shadows::new([Shadow::new()
            .inner()
            .y(-2)
            .blur(1)
            .color(hsluv!(0, 0, 0, 3))]))
        .focus(true)
        .on_change(super::set_new_query)
        .label_hidden("Start typing a song title/artist")
        .placeholder(
            Placeholder::new("Start typing a song title/artist")
                .s(Font::new().italic().color(hsluv!(0, 0, 60.3))),
        )
        .on_key_down_event(|event| {
            event.if_key(Key::Enter, super::add_track);
            event.if_key(Key::Other(" ".to_string()), super::search)
        })
        .text_signal(super::new_query().signal_cloned())
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
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
       // .item(search_track())
        .item_signal(super::tracks_exist().map_true(tracks))
        .item_signal(super::tracks_exist().map_true(panel_footer))
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

// ------ Search ------
// fn search_results() -> impl Element {

// }
