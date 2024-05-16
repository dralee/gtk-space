
/*
* switch button demo
* 2024.05.16 by dralee
*/
use gtk::prelude::*;
use gtk::{glib,Application,ApplicationWindow, Switch, Box, Orientation, Label};
use glib::clone;

const APP_ID: &str = "org.gtk_rs.SwitchDemo";

fn main() -> glib::ExitCode {
    // create a application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app:&Application) {
    // create a label
    let label_tip = Label::builder()
        .label("Switch status:")
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .build();
    let label_val = Label::builder()
        .label("On")
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .build();

    // create a switch
    let switch = Switch::builder()
        .active(true)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // create a switch
    let switch2 = Switch::builder()
        .active(true)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    switch.bind_property("active", &switch2, "active")
        .bidirectional() // double direct
        .build();

    switch.connect_active_notify(clone!(@weak label_val, @weak switch => move |_|{
        print!("switch is active: {}", switch.is_active());
        label_val.set_label(if switch.is_active() {"On"} else {"Off"});
    }));
    
    let l_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let v_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    l_box.append(&label_tip);
    l_box.append(&label_val);
    v_box.append(&l_box);
    v_box.append(&switch);
    v_box.append(&switch2);
    
    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Switch demo")
        .child(&v_box)
        .build();

    // present
    window.present();
}
