/*
* custom window
* load window state and constructed and save it when close the window
* 2024.05.17 by dralee
*/
mod imp;

use gio::Settings;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, gio, Application};

use crate::APP_ID;

const WINDOW_WIDTH:&str = "window-width";
const WINDOW_HEIGHT:&str = "window-height";
const IS_MAXIMIZED:&str = "is-maximized";
const INPUT_CONTENT:&str = "content";

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget,gtk::Native,gtk::Root,gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // create a window
        Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("\"settings\" should not be set before calling \"setup_settings\".");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("\"settings\" should not be set before calling \"setup_settings\".")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // get the size
        let size = self.default_size();
        println!("+++++++++++++++++========================");
        // set the window state in settings
        self.settings().set_int(WINDOW_WIDTH, size.0)?;
        self.settings().set_int(WINDOW_HEIGHT, size.1)?;
        self.settings().set_boolean(IS_MAXIMIZED, self.is_maximized())?;        
        let title = self.title().unwrap();
        println!("title: {}", title);
        self.settings().set_string(INPUT_CONTENT, title.as_str())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // get the window state from settings
        let width = self.settings().int(WINDOW_WIDTH);
        let height = self.settings().int(WINDOW_HEIGHT);
        let is_maximized = self.settings().boolean(IS_MAXIMIZED);
        let content = self.settings().string(INPUT_CONTENT);
        
        println!("content: {}", content);

        // set title
        self.set_title(Some(content.as_str()));
        
        
        // set the size of the window
        self.set_default_size(width, height);

        // if the window was maximized when it closed, maximized it again
        if is_maximized {
            self.maximize();
        }
    }
}