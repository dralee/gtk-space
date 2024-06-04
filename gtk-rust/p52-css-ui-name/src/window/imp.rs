/*
* todo list window
* 2024.05.24 by dralee
*/
use std::cell::{OnceCell, RefCell};
use std::fs::File;

use gtk::prelude::ListModelExtManual;
use gtk::{ ListView, Entry};
use gtk::{glib, gio, CompositeTemplate};
use gtk::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gio::{ListStore, Settings};

use crate::task_object::{TaskData, TaskObject};
use crate::utils::data_path;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/dralee/TodoListMenu/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub task_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<ListStore>>,
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // create action to remove done tasks and add to action group win
        klass.install_action("win.remove-done-tasks", None, |window, _, _|{
            window.remove_done_tasks();
        })
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
        obj.setup_settings();
        obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_actions();
    }
}

// shared by all widgets
impl WidgetImpl for Window {
    
}

// shared by all windows
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        // store task data in vector
        let backup_data: Vec<TaskData> = self.obj()
            .tasks()
            .iter::<TaskObject>()
            .filter_map(Result::ok)
            .map(|task_object| task_object.task_data())
            .collect();

        // save state to file
        let file = File::create(data_path()).expect("could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("could not write data to json file");

        // pass close request on to the parent
        self.parent_close_request()
    }
}

// shared by all application windows
impl ApplicationWindowImpl for Window {
    
}