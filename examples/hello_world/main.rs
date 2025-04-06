use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = Window::new("First Window", Layout::new("d:c,w:30,h:9"), window::Flags::Sizeable);
    win.add(Label::new("Hello World !",Layout::new("d:c,w:13,h:1")));
    app.add_window(win);
    app.run();
    Ok(())
}