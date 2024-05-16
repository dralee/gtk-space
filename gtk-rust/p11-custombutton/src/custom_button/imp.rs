/*
* custom button by self define click action
* adding signals to custom GObjects
* 2024.05.16 by dralee
*/
use std::cell::Cell;
use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::sync::OnceLock;
use glib::subclass::Signal;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton{
    #[property(get,set)]
    number: Cell<i32>,
}

// the central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkCustomButton";
    type Type = super::CustomButton; //super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by alll GObjects
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        //self.obj().set_label(&self.number.get().to_string());
        // bind label to number
        // SYNC_CREATE ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
    
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();        
        SIGNALS.get_or_init(||{
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build()]
        })
    }
}

// Trait shared by all wdidgets
impl WidgetImpl for CustomButton {}

static MAX_NUMBER:i32 = 8;

// 如果我们现在按下按钮，其标签的数量会增加，直到达到 MAX_NUMBER 。
//然后它发出“max-number-reached”信号，我们可以很好地连接到该信号。现在，每当我们收到“max-number-reached”信号时，随附的数字就会打印到标准输出。
// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        // self.number.set(self.number.get() + 1);
        // self.obj().set_label(&self.number.get().to_string());
        let incremented_number = self.obj().number() + 1;
        let obj = self.obj();
        // if number reached MAX_NUMBER,
        // emit max-number-reached signal and set number back to 0
        if incremented_number == MAX_NUMBER {
            obj.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            obj.set_number(0);
        }else {
            obj.set_number(incremented_number);                        
        }
    }
}


