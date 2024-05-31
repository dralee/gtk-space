/*
* css, inteface builder
* 2024.05.31 by dralee
*/
mod window;

use gtk::glib;
use gtk::prelude::ApplicationExtManual;
use gtk::Application;
use gtk::prelude::*;

use window::Window;

const APP_ID:&str = "org.dralee.css-ui";

fn main() -> glib::ExitCode {
    // create an app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app: &Application) {
    // create a new window and present it
    let window = Window::new(app);
    window.present();
}
