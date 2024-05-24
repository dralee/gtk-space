/*
* todo list widget mod
* 2024.05.24 by dralee
*/
mod imp;

use gtk::glib;
use glib::Object;

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder().property("completed", completed)
            .property("content", content)
            .build()
    }
}

// task data
#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}