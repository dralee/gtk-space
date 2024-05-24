/*
* composite template window imp use custombutton
* 2024.05.24 by dralee
*/
use gtk::{prelude::*, template_callbacks};
use gtk::{glib, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::GString;

use crate::custom_button::CustomButton;


#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/CompositeTemplate4/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<CustomButton>,
    #[template_child]
    pub button2: TemplateChild<CustomButton>,
    #[template_child]
    pub button3: TemplateChild<CustomButton>,
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(button:&CustomButton){
        // set the label to "Hello World" after the button has been clicked
        button.set_label("Hello World!");
        println!("*********************** hello.....");
    }

    #[template_callback]
    fn handle_button2_clicked(button:&CustomButton){
        // set the label to "Byte"/Welldone after the button has been clicked
        let mut txt = button.label().unwrap().to_owned();
            if txt == "Welldone" {
                txt = GString::from("Byte");
            } else {
                txt = GString::from("Welldone");
            }
            button.set_label(txt.as_str());
    }

    #[template_callback]
    fn handle_button3_clicked(button:&CustomButton){
        // exit the app after the button has been clicked
        println!("byte!");
        let window = button.parent().and_downcast::<gtk::ApplicationWindow>().unwrap();        
        window.close();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks(); // bind callbacks
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}


impl ObjectImpl for Window {
    fn constructed(&self) {
        // call constructed on parent
        self.parent_constructed();
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