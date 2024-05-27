/*
* todo list window
* 2024.05.24 by dralee
*/
use std::cell::RefCell;
use gtk::{ ListView, Entry};
use gtk::{glib, gio, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gio::ListStore;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/TodoList/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub task_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
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

        // setup
        let obj = self.obj();
        obj.setup_tasks();
        obj.setup_callbacks();
        obj.setup_factory();
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