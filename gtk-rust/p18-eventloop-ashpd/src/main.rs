
/*
* main event loop only blocked the button not all the ui
* use ashpd on linux
* 2024.05.17 by dralee
*/
use ashpd::desktop::account::UserInformation;
use ashpd::WindowIdentifier;
use gtk::glib::{spawn_future_local, clone};
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.EventLoopAshpd";

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
            // get native of button for window identifier
            let native = button.native().expect("Need to be able to get native.");
            // get window identifier so that the dialog will be modal to the main window
            let identifier = WindowIdentifier::from_native(&native).await;
            let request = UserInformation::request()
                .reason("App would like to access user information.")
                .identifier(identifier)
                .send()
                .await;

            if let Ok(response) = request.and_then(|r| r.response()){
                println!("User name: {}", response.name());
            } else {
                println!("Could not get user information.");
            }
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
