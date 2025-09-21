use appcui::prelude::*;
use appcui::ui::appbar::*;

mod countries;
mod music;
mod words;
mod salaries;
mod planets;
mod animals;
mod greek_letters;
mod custom_filter;
mod custom_paint;
mod files;
mod tasks;

const LOGO: [&str; 6] = [
    "██╗     ██╗███████╗████████╗██╗   ██╗██╗███████╗██╗    ██╗",
    "██║     ██║██╔════╝╚══██╔══╝██║   ██║██║██╔════╝██║    ██║",
    "██║     ██║███████╗   ██║   ██║   ██║██║█████╗  ██║ █╗ ██║",
    "██║     ██║╚════██║   ██║   ╚██╗ ██╔╝██║██╔══╝  ██║███╗██║",
    "███████╗██║███████║   ██║    ╚████╔╝ ██║███████╗╚███╔███╔╝",
    "╚══════╝╚═╝╚══════╝   ╚═╝     ╚═══╝  ╚═╝╚══════╝ ╚══╝╚══╝ ",
];

#[Desktop(events    = [MenuEvents,DesktopEvents,AppBarEvents], 
          overwrite = OnPaint, 
          commands  = [ShowCountries, ShowMusic, ShowWords, ShowSalaries, ShowPlanets, ShowAnimals, 
                       ShowGreekLetters, ShowCustomFilter, ShowCustomPaint, ShowFiles, ShowTasks,
                       Exit, About, 
                       NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<MenuButton>,
    menu_example: Handle<MenuButton>,
    menu_help: Handle<MenuButton>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_example: Handle::None,
            menu_help: Handle::None,
        }
    }
    fn update_arrange_windows_method(&mut self, method: Option<desktop::ArrangeWindowsMethod>) {
        self.arrange_method = method;
        if let Some(method) = method {
            self.arrange_windows(method);
        }
    }  
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        surface.clear(Character::with_attributes(' ', attr));
        let x = (surface.size().width as i32) - 57;
        let mut y = (surface.size().height as i32)  - 7;
        for line in LOGO {
            surface.write_string(x, y, line, attr, false);
            y += 1;
        }
    }
}
impl DesktopEvents for MyDesktop {
    fn on_update_window_count(&mut self, _count: usize) {
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
    
    fn on_start(&mut self) { 
        // define and register a menu
        self.menu_arrange = self.appbar().add(MenuButton::new("&Windows",menu!("
            class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "),0,Side::Left));
        self.menu_example = self.appbar().add(MenuButton::new("&Example",menu!("
            class: MyDesktop, items:[
                {&Countries,cmd: ShowCountries},
                {&Music,cmd: ShowMusic},
                {&Words,cmd: ShowWords},
                {&Salaries,cmd: ShowSalaries},
                {&Planets,cmd: ShowPlanets},
                {&Animals,cmd: ShowAnimals},
                {'&Greek Letters',cmd: ShowGreekLetters},
                {'&Files',cmd: ShowFiles},
                {&Tasks, cmd: ShowTasks},
                {---},
                {'Custom Filter', cmd: ShowCustomFilter},
                {'Custom Paint', cmd: ShowCustomPaint},
            ]
        "),0,Side::Left));
        self.menu_help = self.appbar().add(MenuButton::new("&Help",menu!("
            class: MyDesktop, items:[
                {&About,cmd: About},
                {E&xit,cmd: Exit},
            ]
        "),0,Side::Left));
    }  
}
impl MenuEvents for MyDesktop {
    fn on_select(&mut self,_menu:Handle<Menu>,_item:Handle<menu::SingleChoice>,command:mydesktop::Commands){
        match command {
            mydesktop::Commands::NoArrange => self.update_arrange_windows_method(None),
            mydesktop::Commands::Cascade => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Cascade)),
            mydesktop::Commands::Vertical => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Vertical)),
            mydesktop::Commands::Horizontal => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Horizontal)),
            mydesktop::Commands::Grid => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Grid)),
            _ => {}
        }
    }
    fn on_command(&mut self, _menu:Handle<Menu>, _item:Handle<menu::Command>,command:mydesktop::Commands) {
        match command {
            mydesktop::Commands::ShowCountries => { 
                self.add_window(countries::Win::new());
            },
            mydesktop::Commands::ShowMusic => { 
                self.add_window(music::Win::new());
            },
            mydesktop::Commands::ShowWords => { 
                self.add_window(words::Win::new());
            },
            mydesktop::Commands::ShowSalaries => { 
                self.add_window(salaries::Win::new());
            },
            mydesktop::Commands::ShowPlanets => { 
                self.add_window(planets::Win::new());
            },
            mydesktop::Commands::ShowAnimals => { 
                self.add_window(animals::Win::new());
            },
            mydesktop::Commands::ShowGreekLetters => { 
                self.add_window(greek_letters::Win::new());
            },
            mydesktop::Commands::ShowCustomFilter => { 
                self.add_window(custom_filter::Win::new());
            },
            mydesktop::Commands::ShowCustomPaint => { 
                self.add_window(custom_paint::Win::new());
            },
            mydesktop::Commands::ShowFiles => { 
                self.add_window(files::Win::new());
            },
            mydesktop::Commands::ShowTasks => { 
                self.add_window(tasks::Win::new());
            },            
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::About => {
                dialogs::message("List View Example", "This is an example of a list view control in AppCUI");
            },     
            _ => { }      
        }
    }


}
impl AppBarEvents for MyDesktop {
    fn on_update(&self,appbar: &mut AppBar) {
        appbar.show(self.menu_example);
        appbar.show(self.menu_arrange);
        appbar.show(self.menu_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).app_bar().build()?.run();
    Ok(())
}
