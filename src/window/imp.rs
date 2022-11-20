use std::cell::RefCell;

use adw::subclass::prelude::{AdwApplicationWindowImpl, WidgetClassSubclassExt};
use gtk::{
    gio::{self, Settings},
    glib::{self, once_cell::sync::OnceCell, subclass},
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl, prelude::*, widget::CompositeTemplateClass,
    },
};

use crate::launcher::LauncherObject;

#[derive(gtk::CompositeTemplate, Default, Debug)]
#[template(resource = "/org/pug/ui/window.ui")]
pub struct Window {
    #[template_child]
    pub launcher_list: gtk::TemplateChild<gtk::ListBox>,
    //#[template_child]
    //pub add_version: gtk::TemplateChild<gtk::Button>,
    #[template_child]
    pub leaflet: gtk::TemplateChild<adw::Leaflet>,
    #[template_child]
    pub back_button: gtk::TemplateChild<gtk::Button>,

    pub settings: OnceCell<Settings>,
    pub launchers: OnceCell<gio::ListStore>,
    pub current_launcher: RefCell<Option<LauncherObject>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
        obj.setup_launchers();
        obj.get_datas();

        obj.setup_callbacks();
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
