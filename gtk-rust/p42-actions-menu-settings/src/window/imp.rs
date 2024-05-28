use std::cell::OnceCell;

/**
 * action state by ui, window impl, menu sate
 * 2024.05.28 by dralee
 */
use gtk::{glib::{self, subclass::InitializingObject},gio::Settings,
 subclass::prelude::*,
 Box, Button, CompositeTemplate, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/ActionsStateUISettings/window.ui")]
pub struct Window {
    #[template_child]
    pub gtk_box: TemplateChild<Box>,
    #[template_child]
    pub label: TemplateChild<Label>,
    #[template_child]
    pub button: TemplateChild<Button>,
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME:&'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj:&InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // add actions
        let obj = self.obj();
        obj.setup_settings();
        obj.setup_actions();
        obj.bind_settings();
    }
}

impl WidgetImpl for Window {
    
}

impl WindowImpl for Window {
    
}

impl ApplicationWindowImpl for Window {
    
}