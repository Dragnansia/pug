use adw::subclass::prelude::{AdwApplicationWindowImpl, WidgetClassSubclassExt};
use gtk::{
    gio::Settings,
    glib::{self, once_cell::sync::OnceCell},
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl, prelude::*, widget::CompositeTemplateClass,
    },
};

#[derive(gtk::CompositeTemplate, Default, Debug)]
#[template(resource = "/org/pug/window.ui")]
pub struct Window {
    #[template_child]
    pub launcher_list: gtk::TemplateChild<gtk::ListBox>,
    #[template_child]
    pub add_version: gtk::TemplateChild<gtk::Button>,

    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

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

        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();

        obj.setup_actions();
    }
}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::signal::Inhibit {
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        gtk::Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
impl WidgetImpl for Window {}
