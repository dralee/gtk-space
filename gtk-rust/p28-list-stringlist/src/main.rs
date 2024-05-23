/*
* custom list box, property expression, stringlist mode
* 2024.05.20 by dralee
*/
use gtk::{
    glib, Application, ApplicationWindow, NoSelection, Label, ListView, PolicyType, 
    ScrolledWindow, SignalListItemFactory, StringList, StringObject, Widget
};
use gtk::{prelude::*, ListItem};

const APP_ID:&str = "org.gtk_rs.ListStringList";

fn main() -> glib::ExitCode {
    // create application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to "activate" signal
    app.connect_activate(build_ui);

    // run the app
    app.run()
}

fn build_ui(app:&Application) {
    // create a StringList with number from 1 to 100_000
    // StringList implements FromIterator<String>
    let model:StringList = (1..=100_000).map(|number| number.to_string()).collect();

    // factory setup
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item|{
        let label = Label::new(None);
        let list_item = list_item.downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        
        list_item.set_child(Some(&label));

        // bind list_item-> iterm-> number to label-label
        list_item.property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });
 
    // selection list
    let selection_model = NoSelection::new(Some(model)); // MultiSection/NoSelection/SingleSelection
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
        .title("StringList ListView")
        .default_width(560)
        .default_height(320)
        .child(&scrolled_window)
        .build();

    // present window
    window.present();
}
