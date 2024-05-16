/*
* custom button
* 2024.05.16 by dralee
*/
use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct CustomButton;

// the central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkCustomButton";
    type Type = super::CustomButton; //super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by alll GObjects
impl ObjectImpl for CustomButton {}

// Trait shared by all wdidgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {}


