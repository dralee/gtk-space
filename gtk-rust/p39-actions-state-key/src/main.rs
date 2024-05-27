/**
 * action state by keyboard
 * 2024.05.27 by dralee
 */
use gtk::{gio::ActionEntry, glib, prelude::*, Application, ApplicationWindow, Button, Label, Box, Orientation, Align};

const APP_ID:&str = "org.gtk_rs.ActionState";

fn main() -> glib::ExitCode {
    // create a app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to the "activate" signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app:&Application) {
    let original_state = 0;
    let label = Label::builder().label(format!("Counter: {original_state}")).build();
    let button = Button::builder().label("Press").action_name("win.count").action_target(&1.to_variant()).build(); // bind count for parameter 1 by press
    
    // create a Box and add button and label to it
    let g_box = Box::builder().orientation(Orientation::Vertical)
        .margin_top(12).margin_bottom(12).margin_start(12).margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    g_box.append(&label);
    g_box.append(&button);
    
    // create a window
    let window = ApplicationWindow::builder()
        .title("action state")
        .width_request(420)
        .application(app)
        .child(&g_box)
        .build();

    // add action "count" to window taking a integer as parameter
    let action_count = ActionEntry::builder("count")
        .parameter_type(Some(&i32::static_variant_type()))
        .state(original_state.to_variant())
        .activate(move |_,action,parameter|{
            // get state
            let mut state = action.state().expect("could not get state.")
                    .get::<i32>().expect("the variant needs to be of type i32.");

            // get parameter
            let parameter = parameter.expect("could not get parameter.")
                .get::<i32>().expect("the variant needs to be of type i32.");

            // increase state by parameter and store state
            state += parameter;
            action.set_state(&state.to_variant());

            // update label with new state
            label.set_label(&format!("Counter: {state}"));
        })
        .build();

    // add action
    window.add_action_entries([action_count]);

    // present window
    window.present();


}
