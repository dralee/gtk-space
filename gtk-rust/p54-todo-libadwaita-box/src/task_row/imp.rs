
/*
* todo list row imp of Box
* 2024.05.24 by dralee
*/
use std::cell::RefCell;


use gtk::glib;
use gtk::{Label, CheckButton, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::Binding;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/dralee/TodoListLibadwaitaBox/task_row.ui")]
pub struct TaskRow {
    #[template_child]
    pub completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub content_label: TemplateChild<Label>,
    // holding the bindings to properties of TaskObject
    pub bindings: RefCell<Vec<Binding>>,
}


#[glib::object_subclass]
impl ObjectSubclass for TaskRow {
    const NAME:&'static str = "TodoTaskRow";
    type Type = super::TaskRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("task-row");
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>){
        obj.init_template();
    }
}

impl ObjectImpl for TaskRow {}

impl WidgetImpl for TaskRow {}

impl BoxImpl for TaskRow{}