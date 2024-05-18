/*
* hello world increase/descrease number
* 2024.05.15 by dralee
*/
use gtk::{prelude::*, ApplicationWindow, Label, Box, Orientation};
use gtk::{glib, glib::clone, Button, Application};
use std::{cell::Cell, rc::Rc};

const APP_ID: &str = "org.gtk-rs.HelloWorld3";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let label:Label = Label::builder()
        .label("number:")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let label_value:Label = Label::builder()
        .label("0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_increase: Button = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease:Button = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let gtk_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    label_box.append(&label);
    label_box.append(&label_value);
    gtk_box.append(&label_box);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // mutable integer
    // let mut number = 0; // can't borrow
    let number = Rc::new(Cell::new(0));

    // increase number by click the button
    // button_increase.connect_clicked(|_| number += 1);
    
    button_increase.connect_clicked(clone!(@strong number, @strong label_value => move |_| {
        number.set(number.get() + 1);
        //println!("number is {}", number.get());
        label_value.set_label(&number.get().to_string());
    }));

    button_decrease.connect_clicked(move |_|{
        number.set(number.get() - 1);
        label_value.set_label(&number.get().to_string());
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello Number")
        .child(&gtk_box)
        .build();

    window.present();
}
