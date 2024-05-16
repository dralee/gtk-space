/*
* use custom button
* 2024.05.16 by dralee
*/
mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.GObjectSubclassingButton";

fn main() -> glib::ExitCode {
    // create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // run the application
    app.run()
}

fn build_ui(app: &Application) {
    // create a button
    let button = CustomButton::with_label("Press me!");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // connect to "clicked" signal of `button`
    button.connect_clicked(move |button|{
        // set label to "Hello World!"
        button.set_label("Hello World!");
    });

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Gtk Button")
        .child(&button)
        .build();

        // present window
    window.present();
}
