use crate::prelude::*;
use crate::ui::imageviewer::initialization_flags::Flags;
use self::components::ScrollBars;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct ImageViewer {
    surface: Surface,
    image: Image,
    render_method: image::RendererType,
    scale: image::Scale,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars,
}
impl ImageViewer {
    pub fn new(image: Image, layout: Layout, render_method: image::RendererType, scale: image::Scale, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags == Flags::ScrollBars {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            surface: Surface::new(image.width(), image.height()),
            x: 0,
            y: 0,
            flags,
            image,
            scale,
            render_method,
            background: None,
            drag_point: None,
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
        };
        obj.update_surface();
        if flags == Flags::ScrollBars {
            let sz = obj.surface.size();
            obj.scrollbars.update(0, 0, sz);
        }
        obj
    }
    pub fn set_image(&mut self, image: Image) {
        self.image = image;
        self.update_surface();
    }
    #[inline(always)]
    pub fn scale(&self) -> image::Scale {
        self.scale
    }
    pub fn set_scale(&mut self, scale: image::Scale) {
        self.scale = scale;
        self.update_surface();
    }
    #[inline(always)]
    pub fn render_method(&self) -> image::RendererType {
        self.render_method
    }
    pub fn set_render_method(&mut self, render_method: image::RendererType) {
        self.render_method = render_method;
        self.update_surface();
    }
    fn update_surface(&mut self) {
        let sz = self.image.render_size(self.render_method, self.scale);
        self.surface.resize(sz);
        self.surface.draw_image(0, 0, &self.image, self.render_method, self.scale);
        let sz = self.surface.size();
        let control_size = self.size();
        self.scrollbars.update(sz.width as u64, sz.height as u64, control_size);
        self.move_scroll_to(self.x, self.y);
    }

    pub fn set_backgound(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }
    pub fn clear_background(&mut self) {
        self.background = None;
    }

    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.size();
        let surface_size = self.surface.size();
        self.x = if surface_size.width <= sz.width {
            0
        } else {
            x.max((sz.width as i32) - (surface_size.width as i32))
        };
        self.y = if surface_size.height <= sz.height {
            0
        } else {
            y.max((sz.height as i32) - (surface_size.height as i32))
        };
        self.x = self.x.min(0);
        self.y = self.y.min(0);
        self.scrollbars.set_indexes((-self.x) as u64, (-self.y) as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }
}
impl OnResize for ImageViewer {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let paint_sz = self.surface.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64,&self.base);
        self.move_scroll_to(self.x, self.y);
    }
}
impl OnPaint for ImageViewer {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }
        surface.draw_surface(self.x, self.y, &self.surface);
    }
}
impl OnKeyPressed for ImageViewer {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.move_scroll_to(self.x + 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.move_scroll_to(self.x - 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Up") => {
                self.move_scroll_to(self.x, self.y + 1);
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.move_scroll_to(self.x, self.y - 1);
                EventProcessStatus::Processed
            }
            key!("Shift+Left") => {
                self.move_scroll_to(0, self.y);
                EventProcessStatus::Processed
            }
            key!("Shift+Right") => {
                self.move_scroll_to(i32::MIN, self.y);
                EventProcessStatus::Processed
            }
            key!("Shift+Up") => {
                self.move_scroll_to(self.x, 0);
                EventProcessStatus::Processed
            }
            key!("Shift+Down") => {
                self.move_scroll_to(self.x, i32::MIN);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Left") => {
                self.move_scroll_to(self.x + self.size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
                self.move_scroll_to(self.x - self.size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Up") | key!("PageUp") => {
                self.move_scroll_to(self.x, self.y + self.size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") | key!("PageDown") => {
                self.move_scroll_to(self.x, self.y - self.size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.move_scroll_to(0, 0);
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.move_scroll_to(i32::MIN, i32::MIN);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for ImageViewer {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let response = match event {
            MouseEvent::Enter => EventProcessStatus::Ignored,
            MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                if (self.flags == Flags::ScrollBars) && (self.has_focus()) {
                    let sz = self.size();
                    if (data.x == sz.width as i32) || (data.y == sz.height as i32) {
                        return EventProcessStatus::Ignored;
                    }
                }
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = None;
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::None => {}
                    MouseWheelDirection::Left => self.move_scroll_to(self.x + 1, self.y),
                    MouseWheelDirection::Right => self.move_scroll_to(self.x - 1, self.y),
                    MouseWheelDirection::Up => self.move_scroll_to(self.x, self.y + 1),
                    MouseWheelDirection::Down => self.move_scroll_to(self.x, self.y - 1),
                };
                EventProcessStatus::Processed
            }
        };
        // if one of the components require a repaint, than we should repaint even if the canvas required us to ignore the event
        if self.scrollbars.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}
