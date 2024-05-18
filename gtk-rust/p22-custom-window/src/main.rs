/*
* custom window
* 2024.05.17 by dralee
*/
mod custom_window;

use custom_window::Window;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, Button, Align, EditableLabel};
use glib::GString;

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
        .build();
    
    // create a window
    let window = Window::new(&app);
    // window.set_title(Some("Hello World"));
    window.set_child(Some(&button));
    window.set_child(Some(&txt));

    button.connect_clicked(clone!(@strong window => move |_|{
        window.close();
    }));

    // txt.connect_changed(clone!(@weak window => move |txt|{
    //     let content = txt.text();
    //     window.set_title(Some(content.as_str()));
    // }));
    txt.bind_property("text", &window, "title")
        .transform_from(|_, txt: GString|{
            println!("from txt: {}", txt);
            let content = txt.as_str();
            Some(content.to_owned())
        })
        .transform_to(|_, title:GString|{
            println!("to title: {}", title);
            Some(title.as_str().to_owned())
        })
        .bidirectional()
        //.sync_create()
        .build();

    // window.connect_title_notify(|window|{
    //     println!("window title: {}", window.title().unwrap());
    // });
    // 差title初始化时，给txt赋值方向

    // present window
    window.present();
}
