use crate::{
    graphics::{Character, Point, Rect, Size, SpecialChar, Surface, TextAlignament, WrapType},
    prelude::TextFormatBuilder,
};

use super::Theme;

pub(crate) struct ToolTip {
    visible: bool,
    text_pos: Point,
    arrow_pos: Point,
    arrow_char: SpecialChar,
    canvas: Surface,
}
impl ToolTip {
    pub(crate) fn new() -> Self {
        ToolTip {
            visible: false,
            text_pos: Point::default(),
            arrow_pos: Point::default(),
            arrow_char: SpecialChar::ArrowDown,
            canvas: Surface::new(16, 16),
        }
    }
    #[inline(always)]
    pub(crate) fn is_visible(&self) -> bool {
        self.visible
    }
    pub(crate) fn show(&mut self, text: &str, object_rect: &Rect, screen_size: Size, theme: &Theme) -> bool {
        self.visible = false;

        let mut nr_lines = 0u32;
        let max_width = screen_size.width / 2;
        let mut w = 0u32;
        let mut best_width = 0u32;
        let mut chars_count = 0usize;
        for c in text.chars() {
            chars_count += 1;
            if (c == '\n') || (c == '\r') {
                best_width = best_width.max(w);
                w = 0;
                nr_lines += 1;
                continue;
            }
            w += 1;
            if w > max_width {
                best_width = max_width;
                w = 1; // the extra character is moved to the next line
                nr_lines += 1;
            }
        }
        if w > 0 {
            best_width = best_width.max(w);
            nr_lines += 1;
        }
        nr_lines = nr_lines.min(screen_size.height / 3).max(1);
        best_width = best_width.max(5) + 2;

        // find best position  (prefer on-top)
        if object_rect.top() >= ((nr_lines + 1) as i32) {
            let cx = object_rect.center_x();
            let mut x = cx - ((best_width / 2) as i32);
            let top = object_rect.top();
            //let best_x = x;
            x = x.min((screen_size.width as i32) - (best_width as i32)).max(0);
            self.arrow_pos = Point::new(cx.clamp(0, (screen_size.width as i32) - 1), top - 1);
            self.arrow_char = SpecialChar::ArrowDown;
            self.text_pos = Point::new(x, top - ((nr_lines + 1) as i32));
            let format = TextFormatBuilder::new()
                .position(1, 0)
                .attribute(theme.tooltip.text)
                .align(TextAlignament::Left)
                .chars_count(chars_count as u16)
                .wrap_type(WrapType::WordWrap((best_width - 2) as u16))
                .build();
            self.canvas.resize(Size::new(best_width, nr_lines));
            self.canvas.clear(Character::with_attributes(' ', theme.tooltip.text));
            self.canvas.write_text(text, &format);
            self.visible = true;
            return true;
        }
        // bottom position
        if (object_rect.bottom() + ((nr_lines + 1) as i32)) <= screen_size.height as i32 {
            let cx = object_rect.center_x();
            let mut x = cx - ((best_width / 2) as i32);
            let bottom = object_rect.bottom();
            //let best_x = x;
            x = x.min((screen_size.width as i32) - (best_width as i32)).max(0);
            self.arrow_pos = Point::new(cx.clamp(0, (screen_size.width as i32) - 1), bottom + 1);
            self.arrow_char = SpecialChar::ArrowUp;
            self.text_pos = Point::new(x, bottom + 2);
            let format = TextFormatBuilder::new()
                .position(1, 0)
                .attribute(theme.tooltip.text)
                .align(TextAlignament::Left)
                .chars_count(chars_count as u16)
                .wrap_type(WrapType::WordWrap((best_width - 2) as u16))
                .build();
            self.canvas.resize(Size::new(best_width, nr_lines));
            self.canvas.clear(Character::with_attributes(' ', theme.tooltip.text));
            self.canvas.write_text(text, &format);
            self.visible = true;
        }
        false
    }
    pub(crate) fn hide(&mut self) {
        self.visible = false;
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.visible {
            return;
        }
        surface.draw_surface(self.text_pos.x, self.text_pos.y, &self.canvas);
        surface.write_char(
            self.arrow_pos.x,
            self.arrow_pos.y,
            Character::with_attributes(self.arrow_char, theme.tooltip.arrow),
        );
    }
}
