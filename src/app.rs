use nxui::io::storage::Storage;
use nxui::natives_and_messaging::*;
use nxui::widget::menubar::MenuBar;
use nxui::widget::menuitem::MenuItem;
use nxui::widget::textedit::TextEdit;
use nxui::window::{AdvancedOptions, Application, Attributes, Frame};
use nxui::window::dialog::create_dialog;
use crate::about::AboutDialog;

pub struct Editor {

}

impl Editor {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Application for Editor {
    fn app_name(&self) -> String {
        "Xink".to_string()
    }

    fn attributes(&self) -> Attributes {
        Attributes::new(WS_NORMAL, "Xink".to_string(), 750, 600, 10, 10)
    }

    fn is_child_window(&self) -> bool {
        false
    }

    fn startup(&self, _storage: Storage) {
        println!("Startup");
    }

    fn ui(&self, frame: Frame) {
        let edit = TextEdit::new(Attributes::new(WS_NORMAL, "This is Xink!!!!".to_string(), 750, 600, 0, 0), frame);

        edit.show();
        frame.set_menu(initialize_menu());
        frame.show();
        let about = AboutDialog::new(frame);
        create_dialog(Box::new(about));
    }

    fn exit(&self) {
        println!("Exit");
    }
}

fn initialize_menu() -> MenuBar {
    let menu_bar = MenuBar::new();

    // Initialize Menu
    let file_menu = MenuItem::new("File(&F)");
    let edit_menu = MenuItem::new("Edit(&E)");
    let help_menu = MenuItem::new("Help(&H)");

    // Initialize MenuItem
    let new_item = MenuItem::new("New(&N)");
    let open_item = MenuItem::new("Open");
    let save_item = MenuItem::new("Save");
    let save_all_item = MenuItem::new("Save All");
    let close_item = MenuItem::new("Close");
    let export_menu = MenuItem::new("Export");
    let export_to_zip_item = MenuItem::new("Export to Zip");
    let exit_item = MenuItem::new("Exit");

    let undo_item = MenuItem::new("Undo");
    let redo_item = MenuItem::new("Redo");
    let cut_item = MenuItem::new("Cut");
    let copy_item = MenuItem::new("Copy");
    let paste_item = MenuItem::new("Paste");

    let about_item = MenuItem::new("About Xink");

    // Set Menu
    menu_bar.add_item(file_menu);
    menu_bar.add_item(edit_menu);
    menu_bar.add_item(help_menu);

    // Set MenuItem
    file_menu.add_menu(new_item);
    file_menu.add_menuitem(open_item);
    file_menu.add_menuitem(save_item);
    file_menu.add_menuitem(save_all_item);
    file_menu.add_separator();
    file_menu.add_menuitem(close_item);
    file_menu.add_separator();
    file_menu.add_menu(export_menu);
    export_menu.add_menuitem(export_to_zip_item);
    file_menu.add_separator();
    file_menu.add_menuitem(exit_item);

    edit_menu.add_menuitem(undo_item);
    edit_menu.add_menuitem(redo_item);
    edit_menu.add_separator();
    edit_menu.add_menuitem(cut_item);
    edit_menu.add_menuitem(copy_item);
    edit_menu.add_menuitem(paste_item);

    help_menu.add_menuitem(about_item);



    menu_bar
}