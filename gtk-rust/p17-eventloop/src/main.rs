
/*
* main event loop only blocked the button not all the ui
* embed blocking calls in an async context
* 2024.05.17 by dralee
*/
use std::thread;
use std::time::Duration;

use gtk::gio::spawn_blocking;
use gtk::glib::{spawn_future_local, clone};
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};

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

    // connect to "clicked" signal of button
    button.connect_clicked(move |button|{
        spawn_future_local(clone!(@weak button => async move {            
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            let enable_button = spawn_blocking(move ||{
                let five_senconds = Duration::from_secs(5);
                thread::sleep(five_senconds);
                true
            })
            .await
            .expect("Task needs to finish successfully!");
            
            // Activate the button again
            button.set_sensitive(enable_button);
        }));
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
