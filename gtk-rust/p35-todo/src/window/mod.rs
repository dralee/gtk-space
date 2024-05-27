/*
* composite template window
* 2024.05.24 by dralee
*/

mod imp;

use glib::{Object, clone};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};

use crate::task_object::TaskObject;
use crate::task_row::TaskRow;

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

        // wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.tasks()));
        self.imp().task_list.set_model(Some(&selection_model));
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
}