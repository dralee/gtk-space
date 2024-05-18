/*
* list box demo
* 
* 2024.05.18 by dralee
*/
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, ListBox, ScrolledWindow, Label, PolicyType};

const APP_ID:&str = "org.gtk_rs.List1";

fn main() -> glib::ExitCode {
    // create an app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app: &Application) {
    // create a ListBox and add labels with integers from 1 to 100
    let list_box = ListBox::new();
    for number in 1..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    // create a scrolled window with the list box
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // disable horizontal scrolling
        .min_content_width(280)
        .child(&list_box)
        .build();

    // create a window 
    let window = ApplicationWindow::builder()
        .application(app)
        .title("List box")
        .default_width(600)
        .default_height(320)
        .child(&scrolled_window)
        .build();

    // present the window
    window.present();
}
