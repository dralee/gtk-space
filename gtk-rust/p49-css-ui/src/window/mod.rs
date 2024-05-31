/*
* css, gtk css
* 2024.05.31 by dralee
*/
mod imp;

use gtk::{gio,glib::{self, Object}, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gio::ActionGroup, gio::ActionMap,gtk::Native,gtk::ShortcutManager,gtk::Root;
}

impl Window {
    pub fn new(app:&Application ) -> Self {
        Object::builder().property("application", app).build()
    }
}