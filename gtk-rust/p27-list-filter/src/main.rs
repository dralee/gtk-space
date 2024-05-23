/*
* custom list box, ineteger object store list, increasing number, property expression
* 2024.05.20 by dralee
*/
mod integer_object;

use gtk::{
    gio, glib, Application, ApplicationWindow, CustomFilter, CustomSorter, FilterListModel, 
    Label, ListView, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection, 
    SortListModel, Widget
};
use gtk::{prelude::*, ListItem};
use integer_object::IntegerObject;

const APP_ID:&str = "org.gtk_rs.ListIntegerObject4";

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
        let list_item = list_item.downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        
        list_item.set_child(Some(&label));

        // bind list_item-> iterm-> number to label-label
        list_item.property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", Widget::NONE);
    });
 
    // filter
    let filter = CustomFilter::new(move |obj|{
        // get IntegerObject from  glib::Object
        let integer_object = obj.downcast_ref::<IntegerObject>()
            .expect("the object needs to be type of IntegerObject"); 

        // only allow even numbers
        integer_object.number() % 2 == 0
    });

    let filter_mode = FilterListModel::new(Some(model), Some(filter.clone()));

    // sort
    let sorter = CustomSorter::new(move |obj1, obj2|{
        // get IntegerObject from  glib::Object
        let integer_object1 = obj1.downcast_ref::<IntegerObject>()
            .expect("the object needs to be of type IntegerObject.");

            let integer_object2 = obj2.downcast_ref::<IntegerObject>()
            .expect("the object needs to be of type IntegerObject.");

        // get property number from IntegerObject
        let number1 = integer_object1.number();
        let number2 =  integer_object2.number();

        // reverse sorting order -> large numbers come first
        number2.cmp(&number1).into()
    });

    // sort mode
    let sort_mode = SortListModel::new(Some(filter_mode), Some(sorter.clone()));

    // selection list
    let selection_model = SingleSelection::new(Some(sort_mode)); // MultiSection/NoSelection/SingleSelection
    let list_view = ListView::new(Some(selection_model), Some(factory));
    
    // double click the row
    list_view.connect_activate(move |list_view, position|{
        println!("===============position: {}", position);
        // get IntegerObject from ListItem
        let model = list_view.model().expect("the model has to exist.");
        let integer_object = model.item(position)
                .and_downcast::<IntegerObject>()
                .expect("the item has to be an IntegerObject.");

            // increase number of IntegerObject
            integer_object.clone().increase_number();
            let num = integer_object.number();
            println!("now the integer object number: {}", num);

            // notify that the filter and sorter have changed
            filter.changed(gtk::FilterChange::Different);
            sorter.changed(gtk::SorterChange::Different);
    });

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
