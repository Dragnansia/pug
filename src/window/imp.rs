use gtk::{
    glib,
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, WidgetImpl, WindowImpl},
        widget::CompositeTemplateClass,
    },
    ApplicationWindow, CompositeTemplate,
};

#[derive(CompositeTemplate, Default, Debug)]
#[template(resource = "/org/pug/window.ui")]
pub struct Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl WidgetImpl for Window {}
