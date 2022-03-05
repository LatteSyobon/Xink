use nxui::io::storage::Storage;
use nxui::natives_and_messaging::*;
use nxui::widget::textedit::TextEdit;
use nxui::window::{AdvancedOptions, Application, Attributes, Frame};

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
        "Xink Editor".to_string()
    }

    fn attributes(&self) -> Attributes {
        Attributes::new(WS_NORMAL, "Xink Editor".to_string(), 750, 600, 10, 10)
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
        frame.show();
    }

    fn exit(&self) {
        println!("Exit");
    }
}