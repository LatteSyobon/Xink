use nxui::natives_and_messaging::*;
use nxui::widget::button::Button;
use nxui::widget::label::Label;
use nxui::window::{Attributes, Frame};
use nxui::window::dialog::Dialog;

pub struct AboutDialog {
    frame: Frame
}

impl AboutDialog {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame
        }
    }
}

impl Dialog for AboutDialog {
    fn attributes(&self) -> Attributes {
        Attributes::new(WS_NORMAL, "About Xink\0".to_string(), "io.github.lattesyobon.xink.about".to_string(),450, 275, 10, 10)
    }

    fn frame(&self) -> Frame {
        self.frame
    }

    fn ui(&self, frame: Frame) {
        let title = Label::new(Attributes::new(WS_NORMAL, "Xink".to_string(), "".to_string(),100, 30, 10, 10),&frame);
        let nxui = Label::new(Attributes::new(WS_NORMAL, "Powered by NXUI 0.16.0".to_string(), "".to_string(),100, 100, 10, 30),&frame);
        let ok = Button::new(Attributes::new(WS_NORMAL, "OK(&O)".to_string(), "".to_string(),100, 35, 10, 190),&frame);
        ok.show();
        title.show();
        nxui.show();
        frame.show();
    }
}