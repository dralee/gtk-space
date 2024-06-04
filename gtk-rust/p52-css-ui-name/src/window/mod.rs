/*
* composite template window
* 2024.05.24 by dralee
*/

mod imp;

use std::fs::File;

use glib::{Object, clone};
use gtk::subclass::prelude::*;
use gtk::{gio::{self, Settings}, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, CustomFilter, FilterListModel, ListItem};

use crate::task_object::{TaskData, TaskObject};
use crate::task_row::TaskRow;
use crate::utils::data_path;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app:&Application) -> Self {
        // create a window
        Object::builder().property("application", app).build()
    }

    // tasks
    fn tasks(&self) -> gio::ListStore {
        self.imp().tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_tasks(&self) {
        // create new model
        let model = gio::ListStore::new::<TaskObject>();

        // get state and set model
        self.imp().tasks.replace(Some(model));

        // wrap model with filter and selection and pass it to the list view
        let filter_model = FilterListModel::new(Some(self.tasks()), self.filter());        
        let selection_model = NoSelection::new(Some(filter_model.clone()));
        self.imp().task_list.set_model(Some(&selection_model));

        // filter model whenever the value of the key filter changes
        self.settings().connect_changed(Some("filter"), 
            clone!(@weak self as window, @weak filter_model => move |_,_| {
                filter_model.set_filter(window.filter().as_ref());
            })
        );
    }

    // setup callbacks
    fn setup_callbacks(&self) {
        // setup callbacks for activation of the entry
        self.imp().entry.connect_activate(clone!(@weak self as window => move |_|{
            window.new_task();
        }));

        // setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(clone!(@weak self as window => move |_,_|{
            window.new_task();
        }));
    }

    // new task
    fn new_task(&self) {
        // get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // add new task to model
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    // setup factory
    fn setup_factory(&self) {
        // create a new factory
        let factory = SignalListItemFactory::new();

        // create an empty TaskRow during setup
        factory.connect_setup(move |_, list_item|{
            // create TaskRow
            let task_row = TaskRow::new();
            list_item.downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_row));
        });

        // tell factory how to bind TaskRow to a TaskObject
        factory.connect_bind(move |_,list_item|{
            // get TaskObject from ListItem
            let task_object = list_item.downcast_ref::<ListItem>()
                .expect("needs to be ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("the item has to be an TaskObject");

            let task_row = list_item.downcast_ref::<ListItem>()
                .expect("needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("the child has to be a TaskRow");

            task_row.bind(&task_object);
        });

        // tell factory how to unbind TaskRow from TaskObject
        factory.connect_unbind(move |_,list_item|{
            // get TaskRow from ListItem
            let task_row = list_item.downcast_ref::<ListItem>()
                .expect("needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("the child has to be a TaskRow");

            task_row.unbind();
        });

        // set the factory of the list view
        self.imp().task_list.set_factory(Some(&factory));
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp().settings.set(settings).expect("settings should not be set before calling setup_settings.");
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("settings should be set in setup_settings.")
    }

    fn setup_actions(&self) {
        // create action from key filter and add to action group win
        let action_filter = self.settings().create_action("filter");
        self.add_action(&action_filter);
    }

    fn remove_done_tasks(&self) {
        let tasks = self.tasks();
        let mut position = 0;
        while let Some(item) = tasks.item(position) {
            let task_object = item.downcast_ref::<TaskObject>()
                .expect("the object needs to be of type TaskObject.");

            if task_object.is_completed() {
                tasks.remove(position);
            } else {
                position += 1;
            }
        }
    }

    fn filter(&self) -> Option<CustomFilter> {
        // get filter_state from settings
        let filter_state:String = self.settings().get("filter");

        // create custom filters
        let filter_open = CustomFilter::new(|obj|{
            let task_object = obj.downcast_ref::<TaskObject>()
                .expect("the object needs to be of type TaskObject.");

            // only allow open tasks
            !task_object.is_completed()
        });

        let filter_done = CustomFilter::new(|obj|{
            let task_object = obj.downcast_ref::<TaskObject>()
                .expect("the object needs to be type of TaskObject.");

            // only allow done tasks
            task_object.is_completed()
        });

        // return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
    }

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // deserialize data from file to vector
            let backup_data:Vec<TaskData> = serde_json::from_reader(file)
                .expect("it should be posible to read backup_data from the json file.");

            // convert Vec<TaskData> to Vec<TaskObject>
            let task_objects:Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();

            // insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
        }
    }
}