/*
* composite template window
* use adwaita theme
* 2024.05.24 by dralee
*/

mod imp;

use std::fs::File;
use adw;

use glib::{Object, clone};
use adw::subclass::prelude::*;
use adw::{prelude::*, ActionRow};
use gtk::{gio::{self, Settings}, glib, NoSelection,};
use gtk::{CheckButton, Align, CustomFilter, FilterListModel};

use crate::task_object::{TaskData, TaskObject};
use crate::utils::data_path;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app:&adw::Application) -> Self {
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
        self.imp().task_list.bind_model(Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let task_object = obj.downcast_ref().expect("the object should be of type TaskObject.");
                let row = window.create_task_row(task_object);
                row.upcast()
            }));

        // filter model whenever the value of the key filter changes
        self.settings().connect_changed(Some("filter"), 
            clone!(@weak self as window, @weak filter_model => move |_,_| {
                filter_model.set_filter(window.filter().as_ref());
            })
        );

        // assure that the task list is only visible when it is supposed to
        self.set_task_list_visible(&self.tasks());
        self.tasks().connect_items_changed(
            clone!(@weak self as window => move |tasks, _, _, _| {
                window.set_task_list_visible(tasks);
            })
        );
    }

    fn set_task_list_visible(&self, tasks: &gio::ListStore){
        self.imp().task_list.set_visible(tasks.n_items() > 0);
    }

    fn create_task_row(self, task_object: &TaskObject) -> ActionRow{
        // create a check button
        let check_button = CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();

        // create a row
        let row = ActionRow::builder()
            .activatable_widget(&check_button)
            .build();
        row.add_prefix(&check_button);

        // bind properties
        task_object.bind_property("completed", &check_button, "active")
            .bidirectional()
            .sync_create()
            .build();

        task_object.bind_property("content", &row, "title")
            .sync_create()
            .build();

        row
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