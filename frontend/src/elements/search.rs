use std::iter;
use zoon::*;

make_event!(BxSearchInput, "bx-search-input" => web_sys::CustomEvent);

pub struct Search<RE: RawEl> {
    raw_el: RE,
}

impl Search<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlElement>::new("bx-search")
        }
    }
}

#[allow(dead_code)]
impl<RE: RawEl> Search<RE> {
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.raw_el = self.raw_el.attr("placeholder", placeholder);
        self
    }

    pub fn size(mut self, size: &str) -> Self where <RE as zoon::RawEl>::DomElement: std::convert::AsRef<zoon::JsValue> {
        self.raw_el = self.raw_el.prop("size", size);
        self
    }

    pub fn label(mut self, label: &str) -> Self where <RE as zoon::RawEl>::DomElement: std::convert::AsRef<zoon::JsValue> {
        self.raw_el = self.raw_el.prop("labelText", label);
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
        self.raw_el = self.raw_el.event_handler(move |event: BxSearchInput| {
            let value = Reflect::get(&event.event.detail(), &"value".into())
                .unwrap_throw()
                .as_string()
                .unwrap_throw();
            on_change(value);
        });
        self
    }

    fn into_type(self) -> Search<RE> {
        Search {
            raw_el: self.raw_el
        }
    }
}

impl<RE: RawEl> Hookable for Search<RE> {}

impl<RE: RawEl> Focusable for Search<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> Styleable<'_> for Search<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> KeyboardEventAware for Search<RE> where RE::DomElement: AsRef<web_sys::HtmlElement> {}

impl<RE: RawEl> UpdateRawEl for Search<RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Search<RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<RE: RawEl> IntoIterator for Search<RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
