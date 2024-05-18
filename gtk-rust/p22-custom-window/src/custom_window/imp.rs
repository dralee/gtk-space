use std::cell::{OnceCell, Cell};

use gtk::gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{ApplicationWindow};
use gtk::glib::{self};

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>, // 只初始化一次
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyCustomWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;    
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for Window {
    
}

impl WindowImpl for Window {
    // save window state right before the window is closed
    fn close_request(&self) -> glib::Propagation {
        // save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window size");
            // allo to invoke other event handlers
            glib::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for Window {
    
}