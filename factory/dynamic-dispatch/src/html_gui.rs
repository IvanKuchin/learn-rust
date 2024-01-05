use crate::gui::{Button, Dialog};

struct HtmlButton {}

impl HtmlButton {
    fn new() -> Self {
        Self {}
    }
}

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button></button>");
    }
}

pub struct HtmlDialog {}

impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton::new())
    }
}
