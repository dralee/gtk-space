/*
* custom window
* 2024.05.17 by dralee
*/
mod custom_window;

use custom_window::Window;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, Button, Align, EditableLabel};

const APP_ID:&str = "org.gtk_rs.CustomWindow1";

fn main()  -> glib::ExitCode {
    // create a app
    let app = Application::builder().application_id(APP_ID).build();

    // connect activate signal
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn build_ui(app:&Application) {
    // create a button
    let button = Button::builder()
        .margin_top(48).margin_bottom(48)
        .margin_start(48).margin_end(48)
        .label("Close")
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    // create a textbox
    let txt = EditableLabel::builder()
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .text("Hello World")
        .build();
    

    // create a window
    let window = Window::new(&app);

    window.set_child(Some(&button));
    window.set_child(Some(&txt));

    button.connect_clicked(clone!(@strong window => move |_|{
        window.close();
    }));

    // present window
    window.present();
}
