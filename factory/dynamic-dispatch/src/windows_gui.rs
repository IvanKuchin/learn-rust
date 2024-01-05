use crate::gui::{Button, Dialog};

struct WinButton {}

impl Button for WinButton {
    fn render(&self) {
        println!("WinButton");
    }
}

pub struct WinDialog {}

impl Dialog for WinDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton{})
    }
}
