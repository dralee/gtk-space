/**
 * action state by ui, menu sate
 * 2024.05.28 by dralee
 */
mod imp;

use gtk::{gio::{self, ActionEntry, PropertyAction}, 
glib::{self, Object}, prelude::*, subclass::prelude::*, Application,Orientation};

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

        // add property action "button-frame" to window
        let button = self.imp().button.get();
        let action_button_frame = PropertyAction::new("button-frame", &button, "has-frame");
        self.add_action(&action_button_frame);

        // add stateful action "orientation" to window taking a string as parameter
        let action_orientation = ActionEntry::builder("orientation")
            .parameter_type(Some(&String::static_variant_type()))
            .state("Vertical".to_variant())
            .activate(move |window: &Self, action, parameter|{
                // get parameter
                let parameter = parameter.expect("could not get parameter.")
                    .get::<String>().expect("the value needs to be of type String.");

                let orientation = match parameter.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!(),
                };

                // set orientation and save state
                window.imp().gtk_box.set_orientation(orientation);
                action.set_state(&parameter.to_variant());
            })
            .build();

            self.add_action_entries([action_count, action_orientation]);
    }
}