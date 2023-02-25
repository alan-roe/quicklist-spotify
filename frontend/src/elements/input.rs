use std::iter;
use zoon::{println, *};

make_event!(BxInput, "input" => web_sys::CustomEvent);

pub struct Input<RE: RawEl> {
    raw_el: RE,
}

impl Input<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlElement>::new("bx-input")
        }
    }
}

#[allow(dead_code)]
impl<RE: RawEl> Input<RE> {
    pub fn placeholder(mut self, placeholder: &str) -> Self where <RE as zoon::RawEl>::DomElement: std::convert::AsRef<zoon::JsValue> {
        self.raw_el = self.raw_el.prop("placeholder", placeholder);
        self
    }

    pub fn value_signal(
        mut self,
        value: impl Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static,
    ) -> Self where <RE as zoon::RawEl>::DomElement: std::convert::AsRef<zoon::JsValue>{
        self.raw_el = self.raw_el.prop_signal("value", value);
        self
    }

    pub fn on_change(mut self, mut on_change: impl FnMut(String) + 'static) -> Self {
        self.raw_el = self.raw_el.event_handler(move |event: BxInput| {
            let value = Reflect::get(&event.event.target().unwrap_throw(), &"value".into())
                .unwrap_throw()
                .as_string()
                .unwrap_throw();
            on_change(value);
        });
        self
    }

    fn into_type(self) -> Input<RE> {
        Input {
            raw_el: self.raw_el
        }
    }
}

impl<RE: RawEl> Hookable for Input<RE> {}

impl<RE: RawEl> Focusable for Input<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> Styleable<'_> for Input<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> KeyboardEventAware for Input<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> UpdateRawEl for Input<RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Input<RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<RE: RawEl> IntoIterator for Input<RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
