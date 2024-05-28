/**
 * action state by ui, menu state, settings
 * 2024.05.28 by dralee
 */
mod window;

use gtk::{glib, prelude::*, Application, gio};

use window::Window;

const APP_ID:&str = "org.gtk_rs.ActionsStateUISettings";

fn main() -> glib::ExitCode {
    // register and include resource
    gio::resources_register_include!("actionsstatesettings.gresource")
        .expect("failed to register resources.");

    // create a new app
    let app  =Application::builder().application_id(APP_ID).build();

    // connect to activate signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app:&Application){
    let window = Window::new(app);
    
    window.present();
}
