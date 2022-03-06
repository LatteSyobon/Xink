#![windows_subsystem = "windows"]

use crate::app::Editor;

mod app;

fn main() {
    let win = Editor::new();
    nxui::nxui::create_new_app(Box::new(win));
}
