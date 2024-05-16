/*
* custom button by self define click action
* 2024.05.16 by dralee
*/
use std::cell::Cell;
use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton{
    #[property(get,set)]
    number: Cell<i32>,
}

// the central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkCustomButton";
    type Type = super::CustomButton; //super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by alll GObjects
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        //self.obj().set_label(&self.number.get().to_string());
        // bind label to number
        // SYNC_CREATE ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all wdidgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        // self.number.set(self.number.get() + 1);
        // self.obj().set_label(&self.number.get().to_string());
        let incremented_number = self.obj().number() + 1;
        self.obj().set_number(incremented_number);
    }
}


