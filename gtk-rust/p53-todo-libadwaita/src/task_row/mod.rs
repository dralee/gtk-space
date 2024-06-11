
/*
* todo list row
* 2024.05.24 by dralee
*/

mod imp;

use gtk::{glib, pango};
use pango::{AttrInt, AttrList };
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TaskRow{
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    // bind
    pub fn bind(&self, task_object:&TaskObject) {
        // get state
        let completed_button = self.imp().completed_button.get();
        let content_label = self.imp().content_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // bind task_object.completed to task_row.completed_button.active
        let completed_button_binding = task_object
                .bind_property("completed", &completed_button, "active")
                .bidirectional()
                .sync_create()
                .build();

        // save binding
        bindings.push(completed_button_binding);

        // bind task_object.content to task_row.content_label.label
        let content_label_binding = task_object
                .bind_property("content", &content_label, "label")
                .bidirectional()
                .sync_create()
                .build();
        bindings.push(content_label_binding);

        // bind task_object.completed to task_row.content_label.attributes
        let content_label_binding = task_object
                .bind_property("completed", &content_label, "attributes")
                .sync_create()
                .transform_to(|_,active|{
                    let attribute_list = AttrList::new();
                    if active {
                        //  active, content of the label will be strikethrough
                        let attribute = AttrInt::new_strikethrough(true);
                        attribute_list.insert(attribute);
                    }
                    Some(attribute_list.to_value())
                })
                .build();

            // save bindings
            bindings.push(content_label_binding);
    }

    // unbind
    pub fn unbind(&self){
        // unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}