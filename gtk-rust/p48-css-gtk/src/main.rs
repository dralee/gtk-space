/*
* css, gtk css
* 2024.05.31 by dralee
*/
use gtk::{glib, Box, Button,};
use gtk::prelude::ApplicationExtManual;
use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;

const APP_ID:&str = "org.dralee.css5";

fn main() -> glib::ExitCode {
    // create an app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app: &Application) {
    // create a button
    let button1 = Button::with_label("Destructive");
    let button2 = Button::with_label("Suggested");

    button1.add_css_class("destructive-action");  // gtk define css
    button2.set_widget_name("suggested-action"); // gtk define css

    // box
    let b_box = Box::builder()
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .spacing(12).build();
    
    b_box.append(&button1);
    b_box.append(&button2);

    // create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("css for button")
        .child(&b_box)        
        .build();
    window.present();
}
