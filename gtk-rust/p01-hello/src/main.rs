/*
* hello world
* 2024.05.15 by dralee
*/
use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // create a new application
    let app = Application::builder().application_id(APP_ID).build();
    
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application){
    // create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Hello World & Gtk rs")
        .build();

    // present the window
    window.present();
}
