use super::*;

pub fn login_window() -> impl Element {
    Column::new()
        .s(Height::fill().max(500))
        .s(Padding::new().y(10))
        .item(
            Column::new()
                .s(Background::new().color(hsluv!(0, 0, 80)))
                .s(Align::center())
                .s(Font::new().color(hsluv!(0, 0, 5.1)))
                .s(Gap::both(10))
                .s(Padding::all(30))
                .s(Gap::both(20))
                .s(RoundedCorners::all(25))
                .item(error())
                .item(login_button()),
        )
}

fn error() -> impl Element {
    El::new().child_signal(super::login_error().signal_cloned())
}

fn login_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Column::new().item(
        Button::new()
            .s(Background::new()
                .color_signal(hovered_signal.map_bool(|| hsluv!(0, 0, 80), || hsluv!(0, 0, 40))))
            .s(Font::new()
                .color(hsluv!(0, 0, 5.1))
                .weight(FontWeight::Bold))
            .s(Padding::new().x(15).y(10))
            .s(RoundedCorners::all(4))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(super::login)
            .label("Log in"),
    )
}
