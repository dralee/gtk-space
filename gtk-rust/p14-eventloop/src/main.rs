/*
* main event loop only blocked the button not all the ui
* 2024.05.17 by dralee
*/
use std::thread;
use std::time::Duration;

use gtk::glib::{spawn_future_local, clone};
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};
use gtk::gio::spawn_blocking;
use async_channel;

const APP_ID: &str = "org.gtk_rs.EventLoopAvoidblock";

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


    // create channel that can hold at most 1 message at a time
    let (sender, receiver) = async_channel::bounded(1);

    // connect to "clicked" signal of button
    button.connect_clicked(move |_|{
        let sender = sender.clone();
        // the long running operation runs now in a separate thread
        println!("begin into sleep....");
        spawn_blocking(move ||{
            // Deactivate the button until the operation is done
            sender.send_blocking(false)
            .expect("The channel needs to be open.");

            let five_seconds = Duration::from_secs(5);
            println!("let me sleep for 5s");
            thread::sleep(five_seconds);
            println!("5 seconds passed!");
            // Activate the button again
            sender.send_blocking(true)
            .expect("The channel needs to be open.");
        });
        println!("after call sleep.");
    });

    // the main loop executes the asynchronous block
    spawn_future_local(clone!(@weak button => async move {
        while let Ok(enable_button) = receiver.recv().await {
            button.set_sensitive(enable_button);            
        }
    }));

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Event loop sleep ui block")
        .child(&button)
        .build();

    // window present
    window.present();
}
