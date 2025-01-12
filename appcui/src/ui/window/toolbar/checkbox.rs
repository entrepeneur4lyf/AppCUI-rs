use crate::{
    graphics::{Character, SpecialChar, Surface, TextAlignament, TextFormatBuilder, WrapType},
    system::{Handle, Theme},
    utils::Caption,
    utils::ExtractHotKeyMethod
};

use super::{AddToToolbar, ItemBase, PaintData, SymbolAttrState, ToolBarItem, Group};

pub struct CheckBox {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
    checked: bool,
}

add_to_toolbar_impl!(CheckBox);

impl CheckBox {
    pub fn new(text: &str, checked: bool) -> Self {
        let mut obj = CheckBox {
            base: ItemBase::new(true),
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
            checked,
        };
        obj.set_content(text);
        obj
    }
    pub fn set_content(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
        self.base.set_width((self.caption.chars_count() + 2) as u16);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        self.caption.text()
    }
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        self.base.request_recompute_layout();
    }
    pub(crate) fn reverse_check(&mut self) {
        self.set_checked(!self.checked);
    }
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let text_attr = st.get_button_attr(theme);
        let x = self.base.get_left();
        let y = self.base.get_y();
        let mut format = TextFormatBuilder::new()
            .position(x + 2, y)
            .attribute(text_attr)
            .align(TextAlignament::Left)
            .wrap_type(WrapType::SingleLineWrap(self.caption.chars_count() as u16))
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.get_hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_string(x, y, "  ", text_attr, false);
        surface.write_text(self.caption.text(), &format);
        if self.checked {
            surface.write_char(
                x,
                y,
                Character::with_attributes(SpecialChar::CheckMark, st.get_attr(theme, theme.symbol.checked)),
            );
        }
    }
    add_toolbaritem_basic_methods!();
}
