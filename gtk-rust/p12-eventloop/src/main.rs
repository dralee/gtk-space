/*
* main event loop blocked ui
* 2024.05.17 by dralee
*/
use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.EventLoopSleep";

fn main() -> glib::ExitCode {
    // create a new app
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // connect to "activate" signal of app
    app.connect_activate(buidl_ui);

    // run the app
    app.run()    
}

fn buidl_ui(app: &Application) {
    // create a button
    let button = Button::builder()
        .label("Press me")
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .build();

    // connect to "clicked" signal of button
    button.connect_clicked(move |_|{
        // GUI is blocked for 5 seconds after the button is pressed
        let five_seconds = Duration::from_secs(5);
        println!("let me sleep for 5s");
        thread::sleep(five_seconds);
        println!("5 seconds passed!");
    });

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Event loop sleep ui block")
        .child(&button)
        .build();

    // window present
    window.present();
}
