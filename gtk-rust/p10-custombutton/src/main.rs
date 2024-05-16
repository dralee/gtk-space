/*
* custom button with property main
* 2024.05.16 by dralee
*/
mod custom_button;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Orientation};
use custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.CustomButtonProp";

fn main() -> glib::ExitCode {
    // create an application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // connect to "activate" signal
    app.connect_activate(build_ui);

    // run the application
    app.run()    
}

fn build_ui(app: &Application) {
    // create a button
    let button1:CustomButton = CustomButton::new();
    let button2:CustomButton = CustomButton::new();

    // create a box 
    let b_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12).margin_bottom(12)
        .margin_start(12).margin_end(12)
        .build();
    
    button1.set_margin_bottom(12);

    b_box.append(&button1);
    b_box.append(&button2);

    // button1的值比button2的值小1
    button1.bind_property("number", &button2, "number")
        // 转换button1的number属性为button2的number属性
        .transform_to(|_, number:i32|{
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        // 绑定button2的number属性为button1的number属性
        .transform_from(|_,number:i32|{
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    button1.connect_number_notify(|button|{
        println!("The current number of button1 is {}", button.number());
    });

    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom button property")
        .child(&b_box)
        .build();

    // present the window
    window.present();
}
