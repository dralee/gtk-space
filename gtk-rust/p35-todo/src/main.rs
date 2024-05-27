/*
* composite template
* 2024.05.23 by dralee
*/
mod task_object;
mod task_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};

use window::Window;

const APP_ID:&str = "org.gtk_rs.TodoList";

fn main() -> glib::ExitCode {
    // register and include resources
    gio::resources_register_include!("todo_list.gresource")
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
