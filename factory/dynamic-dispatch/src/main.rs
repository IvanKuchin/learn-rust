mod gui;
mod html_gui;
mod windows_gui;

use crate::gui::Dialog;
use html_gui::HtmlDialog;
use windows_gui::WinDialog;

fn init(s: &str) -> &dyn Dialog {
    match s {
        "win" => &WinDialog{},
        "html" => &HtmlDialog{},
        _ => panic!("Unknown Dialog type"),
    }
}

fn main() {
    println!("HTML section:");
    let dialog1 = init("html");
    dialog1.render();
    println!("Win section:");
    let dialog2 = init("win");
    dialog2.render();
}
