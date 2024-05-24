/*
* composite template window imp use custombutton
* 2024.05.24 by dralee
*/
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::GString;

use crate::custom_button::CustomButton;


#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/CompositeTemplate3/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<CustomButton>,
    #[template_child]
    pub button2: TemplateChild<CustomButton>,
    #[template_child]
    pub button3: TemplateChild<CustomButton>,
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
        self.button2.connect_clicked(move|button|{
            let mut txt = button.label().unwrap().to_owned();
            if txt == "Welldone" {
                txt = GString::from("Byte");
            } else {
                txt = GString::from("Welldone");
            }
            button.set_label(txt.as_str());
        });
        
        self.button3.connect_clicked(move|button|{ // 点击关闭窗口
            let window = button.parent().and_downcast::<gtk::ApplicationWindow>().unwrap();
            window.close();
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