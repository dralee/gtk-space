/*
* css, only the button text style set.
* 2024.05.31 by dralee
*/
use gtk::{glib, Button, CssProvider};
use gtk::prelude::ApplicationExtManual;
use gtk::{Application, ApplicationWindow};
use gtk::gdk::Display;
use gtk::prelude::*;

const APP_ID:&str = "org.dralee.css2";

fn main() -> glib::ExitCode {
    // create an app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to signal
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn load_css() {
    // load the css file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("could not connect to a display."),
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_ui(app: &Application) {
    // create a button
    let button = Button::builder()
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .label("Press")
        .build();

    // create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("css for button")
        .child(&button)
        .build();
    window.present();
}
