use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod snake_game;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(40, 24)).single_window().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}   