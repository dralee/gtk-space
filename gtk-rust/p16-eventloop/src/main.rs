/*
* main event loop only blocked the button not all the ui
* by the async task
* as the single thread we can even get rid of the channel while achieving the same result
* 2024.05.17 by dralee
*/
use gtk::glib::{spawn_future_local, clone, timeout_future_seconds};
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
            timeout_future_seconds(5).await;
            // Activate the button again
            button.set_sensitive(true);
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
