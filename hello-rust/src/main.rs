/*
* demo show a window
* 2024.05.14 by dralee
*/
use glib::clone;
use gtk::glib;
use gtk::prelude::*;

// When the application is launched
fn on_activate(app: &gtk::Application) {
    // create a new window
    let window = gtk::ApplicationWindow::new(app);
    // with a button in it
    let button = gtk::Button::with_label("Hello World!");
    // which closes the window when clicked
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}

fn main() {
    // create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("org.dralee.hello-rust")
        .build();
    app.connect_activate(on_activate);
    // run the application
    app.run();
}
