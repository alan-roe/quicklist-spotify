use std::{iter, rc::Rc, cell::RefCell, marker::PhantomData};
use zoon::*;

make_event!(BxBtn, "bx-btn" => web_sys::CustomEvent);

make_flags!(OnPress);
pub struct Button<OnPressFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<OnPressFlag>,
}

#[allow(dead_code)]
impl Button<OnPressFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("bx-btn"),
            flags: PhantomData
        }
    }

    pub fn value_signal(
        mut self,
        value: impl Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.prop_signal("value", value);
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.raw_el = self.raw_el.prop("size", size);
        self
    }
}

impl<'a, OnPressFlag, RE: RawEl> Button<OnPressFlag, RE> {
    pub fn label(
        mut self,
        label: impl IntoElement<'a> + 'a,
    ) -> Button<OnPressFlag, RE>
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Button<OnPressFlag, RE>
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn on_press(self, on_press: impl FnMut() + 'static) -> Button<OnPressFlagSet, RE>
    where
        OnPressFlag: FlagNotSet,
    {
        let on_click = Rc::new(RefCell::new(on_press));
        let on_enter_down = on_click.clone();
        self.on_click(move || on_click.borrow_mut()())
            .on_key_down_event(move |event| {
                event.if_key(Key::Enter, || on_enter_down.borrow_mut()())
            })
            .into_type()
    }

    fn into_type<NewOnPressFlag>(self) -> Button<NewOnPressFlag, RE> {
        Button {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag, RE: RawEl> KeyboardEventAware for Button<OnPressFlag, RE> {}

impl<OnPressFlag, RE: RawEl> MouseEventAware for Button<OnPressFlag, RE> {}

impl<OnPressFlag, RE: RawEl> UpdateRawEl for Button<OnPressFlag, RE> {
    type RawEl = RE;
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<OnPressFlag, RE: RawEl> Styleable<'_> for Button<OnPressFlag, RE> {}


impl<OnPressFlag, RE: RawEl + Into<RawElement>> Element for Button<OnPressFlag, RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnPressFlag, RE: RawEl> IntoIterator for Button<OnPressFlag, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
