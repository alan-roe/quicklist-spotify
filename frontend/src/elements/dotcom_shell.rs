use std::iter;
use zoon::*;

pub struct DotcomShell<RE: RawEl> {
    raw_el: RE,
}

#[allow(dead_code)]
impl DotcomShell<RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("dds-content-item")//.prop("has-profile", "{false}").prop("has-search", "{false}"),
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

impl<RE: RawEl> Styleable<'_> for DotcomShell<RE> {}

impl<RE: RawEl> UpdateRawEl for DotcomShell<RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<RE: RawEl + Into<RawElement>> Element for DotcomShell<RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<RE: RawEl> IntoIterator for DotcomShell<RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
