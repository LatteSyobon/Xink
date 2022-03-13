#![windows_subsystem = "windows"]

use crate::app::Editor;

mod app;
mod about;

fn main() {
    nxui::initialize();
    let win = Editor::new();
    nxui::nxui::create_new_app(Box::new(win));
}
