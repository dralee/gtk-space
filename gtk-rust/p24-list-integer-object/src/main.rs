/*
* custom list box, ineteger object store list
* 2024.05.18 by dralee
*/
mod integer_object;

use gtk::{
    gio, glib, Application, ApplicationWindow, Label, ListView,
    PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection,
};
use gtk::{prelude::*, ListItem};
use integer_object::IntegerObject;

const APP_ID:&str = "org.gtk_rs.ListIntegerObject";

fn main() -> glib::ExitCode {
    // create application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app:&Application) {
    // create a Vec<IntegerObject> with number from 1 to 100_000
    let vector:Vec<IntegerObject> = (1..=100_000).map(IntegerObject::new).collect();

    // create a model
    let model = gio::ListStore::new::<IntegerObject>();

    // add the vector to the model
    model.extend_from_slice(&vector);

    // factory setup
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item|{
        let label = Label::new(None);
        list_item.downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });

    // factory bind
    factory.connect_bind(move |_,list_item|{
        // get IntegerObject from ListItem
        let integer_object = list_item
            .downcast_ref::<ListItem>()
            .expect("needs to be ListItem")
            .item()
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an IntegerObject");

        // get Label from ListItem
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("needs to be ListItem")
            .child()
            .and_downcast::<Label>()
            .expect("The child has to be a Label.");

        // set label to number
        label.set_text(&integer_object.number().to_string());
    });

    // selection list
    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    // scrolled window
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .max_content_width(320)
        .child(&list_view)
        .build();

    // window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Integer Object ListView")
        .default_width(560)
        .default_height(320)
        .child(&scrolled_window)
        .build();

    // present window
    window.present();
}
