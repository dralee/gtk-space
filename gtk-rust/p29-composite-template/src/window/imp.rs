/*
* composite template window imp
* 2024.05.20 by dralee
*/
use gtk::prelude::*;
use gtk::{glib, Button, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/CompositeTemplate1/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
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
    fn constructed(&self) {
        // call constructed on parent
        self.parent_constructed();

        // connect to clicked signal of button
        self.button.connect_clicked(move |button|{
            button.set_label("Hello World!");
        });
    }
}

// shared by all widgets
impl WidgetImpl for Window {
    
}

// shared by all windows
impl WindowImpl for Window {
    
}

// shared by all application windows
impl ApplicationWindowImpl for Window {
    
}