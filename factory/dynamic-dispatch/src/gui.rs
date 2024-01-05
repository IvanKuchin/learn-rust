pub trait Button {
    fn render(&self);
}

pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;
    fn render(&self) {
        let butt_ok = self.create_button();
        let butt_cancel = self.create_button();

        println!("Dialog rendering");
        print!("Button ok:");
        butt_ok.render();
        print!("Button cancel:");
        butt_cancel.render();
    }
}