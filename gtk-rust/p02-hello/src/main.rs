/*
* hello world with button
* 2024.05.15 by dralee
*/
use gtk::prelude::*;
use gtk::{glib, Application, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // create a new application
    let app = Application::builder().application_id(APP_ID).build();
    
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application){
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of button
    button.connect_clicked(|btn|{
        let txt = btn.label().unwrap_or_default();
        if txt == "Press me!"
        {
            btn.set_label("Hello World");
        }
        // set the label to "Hello World"
        else {
            btn.set_label("Press me!");
        }
    });

    // create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Hello World & Gtk rs")
        .child(&button)
        .build();

    // present the window
    window.present();
}
