/**
 * action state by ui
 * 2024.05.27 by dralee
 */
mod imp;

use gtk::{gio::{self, ActionEntry}, glib::{self, Object}, prelude::*, subclass::prelude::*, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app:&Application) -> Self{
        Object::builder().property("application", app).build()
    }

    pub fn setup_actions(&self) {
        // add stateful action `count`  to window taking an integer as parameter
        let original_state = 0;
        let action_count = ActionEntry::builder("count")
            .parameter_type(Some(&i32::static_variant_type()))
            .state(original_state.to_variant())
            .activate(move |window:&Self, action, parameter|{
                let mut state = action.state().expect("could not get state.")
                    .get::<i32>().expect("the variant needs to be of type i32.");

                let parameter = parameter.expect("could not get parametr.")
                    .get::<i32>().expect("the variant needs to be of type i32.");

                // increase the state by parameter
                state += parameter;
                action.set_state(&state.to_variant());

                // update label with state
                window.imp().label.set_label(&format!("Counter: {}", state));
            })
            .build();

        self.add_action_entries([action_count]);
    }
}