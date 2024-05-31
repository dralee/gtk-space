/*
* css, custom button css
* 2024.05.31 by dralee
*/
use gtk::{glib, Box, Button, CssProvider};
use gtk::prelude::ApplicationExtManual;
use gtk::{Application, ApplicationWindow};
use gtk::gdk::Display;
use gtk::prelude::*;

const APP_ID:&str = "org.dralee.css3";

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
    let button1 = Button::with_label("Press");
    let button2 = Button::with_label("Press 2");

    button1.add_css_class("button-1"); // button.button-1
    button2.add_css_class("button-2"); // button.button-1

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
