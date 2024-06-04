/*
* composite template
* 2024.05.30 by dralee
*/
mod task_object;
mod task_row;
mod utils;
mod window;

use gtk::gdk::Display;
use gtk::prelude::ApplicationExtManual;
use gtk::{prelude::*, CssProvider};
use gtk::{gio, glib, Application};

use window::Window;

const APP_ID:&str = "org.dralee.TodoListMenu";

fn main() -> glib::ExitCode {
    // register and include resources
    gio::resources_register_include!("todo_list_menu.gresource")
        .expect("failed to register resources.");

    // create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal
    app.connect_startup(|app|{
        setup_shortcurts(app);
        load_css();
    });
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn load_css(){
    let provider = CssProvider::new();
    provider.load_from_resource("/org/dralee/TodoListMenu/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("could not connect to a display"),
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn setup_shortcurts(app:&Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &Application) {    
    // create a window
    let window = Window::new(app);

    // present window
    window.present();
}