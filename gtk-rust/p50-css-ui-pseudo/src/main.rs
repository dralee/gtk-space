/*
* css, inteface builder
* 2024.05.31 by dralee
*/
mod window;

use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::ApplicationExtManual;
use gtk::Application;
use gtk::prelude::*;

use gtk::CssProvider;
use window::Window;

const APP_ID:&str = "org.dralee.cssPseudoUI";

fn main() -> glib::ExitCode {
    // create an app
    let app = Application::builder().application_id(APP_ID).build();

    // connect to signal
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // run app
    app.run()
}

fn load_css(){
    // load css file and add to provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    
    // add the provider to the default screen
    gtk::style_context_add_provider_for_display(&Display::default().expect("could not connect to a display"),
     &provider, 
     gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_ui(app: &Application) {
    // create a new window and present it
    let window = Window::new(app);
    window.present();
}
