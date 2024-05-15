/*
* hello world increase number
* 2024.05.15 by dralee
*/
use gtk::{prelude::*, ApplicationWindow};
use gtk::{glib, Button, Application};
use std::cell::Cell;

const APP_ID: &str = "org.gtk-rs.HelloWorld3";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button_increase: Button = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // mutable integer
    // let mut number = 0; // can't borrow
    let number = Cell::new(0);

    // increase number by click the button
    // button_increase.connect_clicked(|_| number += 1);
    button_increase.connect_clicked(move |_| {
        number.set(number.get() + 1);
        println!("number is {}", number.get());
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello Number")
        .child(&button_increase)
        .build();

    window.present();
}
