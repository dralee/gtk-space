/*
* css, gtk css
* 2024.05.31 by dralee
*/

use gtk::{glib, CompositeTemplate};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;

#[derive(Default, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct Window;

#[glib::object_subclass]
impl  ObjectSubclass for Window {
    const NAME:&'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {  
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    
}

impl WidgetImpl for Window {
    
}

impl WindowImpl for Window {
    
}

impl ApplicationWindowImpl for Window {
    
}