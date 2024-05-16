/*
* custom button by self define click action
* 2024.05.16 by dralee
*/
use std::cell::Cell;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct CustomButton{
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
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.number.get().to_string());
    }
}

// Trait shared by all wdidgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.number.set(self.number.get() + 1);
        self.obj().set_label(&self.number.get().to_string());
    }
}


