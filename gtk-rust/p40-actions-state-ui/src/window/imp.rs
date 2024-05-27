/**
 * action state by ui, window impl
 * 2024.05.27 by dralee
 */
use gtk::{glib::{self, subclass::InitializingObject}, subclass::prelude::*, CompositeTemplate, Label};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/ActionsStateUI/window.ui")]
pub struct Window {
    #[template_child]
    pub label: TemplateChild<Label>,
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
        self.obj().setup_actions();
    }
}

impl WidgetImpl for Window {
    
}

impl WindowImpl for Window {
    
}

impl ApplicationWindowImpl for Window {
    
}