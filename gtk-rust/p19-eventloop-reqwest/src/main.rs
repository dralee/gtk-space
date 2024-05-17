
/*
* main event loop only blocked the button not all the ui
* use reqwest to request the http://www.gtk-rs.org content
* 2024.05.17 by dralee
*/
use std::sync::OnceLock;
use gtk::glib::{spawn_future_local, clone};
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};
use async_channel;
use tokio::runtime::Runtime;

const APP_ID: &str = "org.gtk_rs.EventLoopAshpd";

// 注入tokio runtime
fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(||{
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

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

    let (sender,receiver) = async_channel::bounded(1);

    // connect to "clicked" signal of button
    button.connect_clicked(move |_|{
        // the main loop executes the asyncronous block
        runtime().spawn(clone!(@strong sender => async move {
            let response = reqwest::get("https://www.gtk-rs.org").await;
            sender.send(response).await.expect("the channel needs to be open.");            
        }));
    });

    // the main loop executes the asyncronous block
    spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                println!("Status: {}", response.status());
            } else {
                println!("Could not make a `GET` request.");
            }
        }
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
