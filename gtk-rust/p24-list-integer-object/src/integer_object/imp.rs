/*
* custom list box, ineteger object store list
* 2024.05.18 by dralee
*/
use std::cell::Cell;

use gtk::glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::glib;
use glib::Properties;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::IntegerObject)]
pub struct IntegerObject {
    #[property(get, set)]
    number: Cell<i32>,
}

// the central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for IntegerObject {
    const NAME: &'static str = "ListBoxIntegerObject";
    type Type = super::IntegerObject;
}

// trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for IntegerObject {
    
}