use std::iter;
use zoon::*;

make_event!(BxInput, "bx-input" => web_sys::CustomEvent);

pub struct Input<RE: RawEl> {
    raw_el: RE,
}

#[allow(dead_code)]
impl Input<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("bx-input"),
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.raw_el = self.raw_el.prop("placeholder", placeholder);
        self
    }

    pub fn value_signal(
        mut self,
        value: impl Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.prop_signal("value", value);
        self
    }

    pub fn on_change(mut self, mut on_change: impl FnMut(String) + 'static) -> Self {
        self.raw_el = self.raw_el.event_handler(move |event: BxInput| {
            let value = Reflect::get(&event.event.detail(), &"value".into())
                .unwrap_throw()
                .as_string()
                .unwrap_throw();
            on_change(value);
        });
        self
    }
}

impl<RE: RawEl> Focusable for Input<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> KeyboardEventAware for Input<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> Styleable<'_> for Input<RE> {}

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
