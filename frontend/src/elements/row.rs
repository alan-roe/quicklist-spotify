use std::iter;
use zoon::*;

pub struct Row<RE: RawEl> {
    raw_el: RE,
}

#[allow(dead_code)]
impl Row<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("div").class("bx--row"),
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

impl ChoosableTag for Row<RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        Self {
            raw_el: RawHtmlEl::new(tag.as_str()).class("bx--row"),
        }
    }
}

impl<RE: RawEl> Styleable<'_> for Row<RE> {}

impl<RE: RawEl> UpdateRawEl for Row<RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Row<RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<RE: RawEl> IntoIterator for Row<RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
