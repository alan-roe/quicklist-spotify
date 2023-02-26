use std::iter;
use zoon::*;

pub struct Column<RE: RawEl> {
    raw_el: RE,
}

#[allow(dead_code)]
impl Column<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("div").class("bx--col")
        }
    }

    pub fn child<'a>(mut self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        self.raw_el = self.raw_el.child(child);
        self
    }

    pub fn child_signal<'a>(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.child_signal(child);
        self
    }

    pub fn children<'a>(
        mut self,
        children: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Self {
        self.raw_el = self.raw_el.children(children);
        self
    }

    pub fn children_signal_vec<'a>(
        mut self,
        children: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.children_signal_vec(children);
        self
    }
}

impl<RE: RawEl> Styleable<'_> for Column<RE> {}

impl ChoosableTag for Column<RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("bx--col")
        }
    }
}

impl<RE: RawEl> UpdateRawEl for Column<RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Column<RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<RE: RawEl> IntoIterator for Column<RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
