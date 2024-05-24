/*
* composite template
* 2024.05.23 by dralee
*/
mod window;
mod custom_button;

use gtk::prelude::*;
use gtk::{gio, glib, Application};

use window::Window;

const APP_ID:&str = "org.gtk_rs.CompositeTemplate3";

fn main() -> glib::ExitCode {
    // register and include resources
    gio::resources_register_include!("composite_template3.gresource")
        .expect("failed to register resources.");

    // create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app: &Application) {    
    // create a window
    let window = Window::new(app);

    // present window
    window.present();
}
