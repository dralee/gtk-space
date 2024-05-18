/*
* custom list box, ineteger object store list
* 2024.05.18 by dralee
*/
mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }
}
