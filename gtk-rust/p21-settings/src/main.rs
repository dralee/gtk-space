/*
* use the settings for save the switch is enable "is-switch-enabled"
* 2024.05.17 by dralee
*/
use gtk::{glib, Align, Application, ApplicationWindow, Switch};
use gtk::prelude::*;
use gtk::gio::Settings;

const APP_ID:&str = "org.gtk_rs.Settings1";

fn main() -> glib::ExitCode {
    // create a app
    let app = Application::builder().application_id(APP_ID).build();

    // connect the activate signal of the app
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app: &Application) {    
    let key:&str = "is-switch-enabled";
    // initialize settings
    let settings = Settings::new(APP_ID);
    // get the last switch state from the settings
    let is_switch_enabled = settings.boolean(key);
    println!("read switch state settings: {}", is_switch_enabled);

    // create a switch
    let switch = Switch::builder()
        .margin_top(48).margin_bottom(48)
        .margin_start(48).margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();

    switch.connect_state_set(move |_, is_enable|{
        println!("switch state: {}", is_enable);
        // save the switch state to the settings        
        settings.set_boolean(key, is_enable)
        .expect("Could not set setting.");
        // allow to invoke other event handlers
        glib::Propagation::Proceed
    });

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Switch Settings")
        .child(&switch)
        .build();

    // present the window
    window.present();
}
