use appcui::prelude::*;

#[Window(events = MenuEvents, commands=Increment)]
struct MyWin {
    m_counter: Handle<menu::Command>,
    some_menu: Handle<menubar::MenuEntry>,
    counter: u32,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Test,a:c,w:40,h:8"),
            m_counter: Handle::None,
            some_menu: Handle::None,
            counter: 0
        };
        let mut m = Menu::new();
        w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
        w.some_menu = w.menubar_mut().add(menubar::MenuEntry::new("Some menu",m,0,menubar::MenuBarPosition::Left));

        w
    }
}
impl MenuEvents for MyWin {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.show(self.some_menu);
    }
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
        if item == self.m_counter {
            self.counter += 1;
            let new_text = format!("Increment ({})",self.counter);
            if let Some(menuitem) = self.menuitem_mut(menu, item) {
                menuitem.set_caption(new_text.as_str());
            }
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().menu_bar().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
