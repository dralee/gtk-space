/**
 * custom action group
 * Ctrl+W to close the window
 * 2024.05.27 by dralee
 */
use gtk::{gio::{ActionEntry, SimpleActionGroup}, glib::{self, clone}, prelude::*, Application, ApplicationWindow};

const APP_ID:&str = "org.gtk_rs.ActionsCustom";

fn main() -> glib::ExitCode {
    // create a app
    let app = Application::builder().application_id(APP_ID).build();

    // connect activate signal
    app.connect_activate(buid_ui);

    // set keyboard accelerator to trigger "custom-group.close"
    app.set_accels_for_action("custom-group.close", &["<Ctrl>W"]);

    // run the app
    app.run()
}

fn buid_ui(app:&Application) {
    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .width_request(420)
        .title("Custom action group")
        .build();

    // add action "close" to window
    let action_close = ActionEntry::builder("close")
        .activate(clone!(@weak window => move |_, _, _|{
            window.close();        
        }))
        .build();

    // create a new action group and add actions
    let actions = SimpleActionGroup::new();
    actions.add_action_entries([action_close]);
    // add group to window
    window.insert_action_group("custom-group", Some(&actions));

    // present window
    window.present();
}
