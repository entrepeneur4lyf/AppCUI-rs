use crate::prelude::*;

#[derive(DropDownListType)]
enum MathOp {
    #[VariantInfo(description = "(Add multiple numbers)", symbol = "∑")]
    Sum,

    #[VariantInfo(description = "(Multiply multiple numbers)", symbol = "∏")]
    Product,

    #[VariantInfo(description = "(Calculate the integral of a function)", symbol = "∫")]
    Integral,

    #[VariantInfo(description = "(Calculate the radical of a number)", symbol = "√")]
    Radical,

    #[VariantInfo(description = "(Check if all elements from a set are different)", symbol = "≠")]
    Different,
}

#[test]
fn check_symbol_paint() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Sum selected')   
        CheckHash(0xFE59E5136C4A00BD) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = DropDownList::<MathOp>::with_symbol(1, Layout::new("x:1,y:1,w:50"), dropdownlist::Flags::ShowDescription);
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Sum selected')   
        CheckHash(0xFE59E5136C4A00BD) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:ShowDescription,symbolsize:1");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_no_symbols() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x1CFE19F59CB0D41A) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:ShowDescription");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_allow_none() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (should be None)')   
        CheckHash(0xC716BFA675C55B4B) 
        Key.Pressed(Space)
        Paint('Opened - None should be selected')   
        CheckHash(0xAA1D368EE131B77B) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:AllowNoneSelection");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_none_with_different_name() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (should be Select Something)')   
        CheckHash(0x6A430F7BD6933C21) 
        Key.Pressed(Space)
        Paint('Opened - Select Something should be selected')   
        CheckHash(0xF30519060AA60813) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:AllowNoneSelection,none:'Select something'");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events=DropDownListEvents<MathOp>,internal: true)]
    struct MyWindow {}
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:0,y:0,w:56,h:7"),
            };
            let mut db = DropDownList::<MathOp>::with_symbol(1, Layout::new("x:1,y:1,w:50"), dropdownlist::Flags::ShowDescription);
            db.add(MathOp::Sum);
            db.add(MathOp::Product);
            db.add(MathOp::Integral);
            db.add(MathOp::Radical);
            db.add(MathOp::Different);
            w.add(db);
            w
        }
    }
    impl DropDownListEvents<MathOp> for MyWindow {
        fn on_selection_changed(&mut self, handle: Handle<DropDownList<MathOp>>) -> EventProcessStatus {
            let s = self
                .control(handle)
                .and_then(|p| p.selected_item())
                .map(|p| p.name())
                .unwrap_or("Invalid")
                .to_string();
            self.set_title(&s);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Paint('Sum selected (on title window)')   
        CheckHash(0x277171A9375683CA) 
        Key.Pressed(Down)
        Paint('Product selected (on title window)')   
        CheckHash(0x7035C003A73ED604) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}
