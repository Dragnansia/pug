use std::cell::RefCell;

use gtk::{
    glib::{self, once_cell::sync::Lazy, ParamSpec, ParamSpecString},
    prelude::ToValue,
    subclass::prelude::{ObjectImpl, ObjectSubclass},
};

#[derive(Default)]
pub struct LauncherObject {
    pub name: RefCell<String>,
    pub icon: RefCell<String>,
    pub package_name: RefCell<Option<String>>,
    pub path: RefCell<String>,
    pub installation_path: RefCell<String>,
}

impl LauncherObject {
    pub fn name(&self) -> String {
        if let Some(package_name) = self.package_name.borrow().clone() {
            format!("{} ({package_name})", self.name.borrow())
        } else {
            self.name.borrow().clone()
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LauncherObject {
    const NAME: &'static str = "LauncherObject";
    type Type = super::LauncherObject;
}

impl ObjectImpl for LauncherObject {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("icon").build(),
                ParamSpecString::builder("package-name").build(),
                ParamSpecString::builder("path").build(),
                ParamSpecString::builder("installation-path").build(),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let input_value = value.get().expect("The value need to be of type `String`");
                self.name.replace(input_value);
            }
            "icon" => {
                let input_value = value.get().expect("The value need to be of type `String`");
                self.icon.replace(input_value);
            }
            "package-name" => {
                let input_value = value
                    .get()
                    .expect("The value need to be of type `Option<String>`");
                self.package_name.replace(input_value);
            }
            "path" => {
                let input_value = value.get().expect("The value need to be of type `String`");
                self.path.replace(input_value);
            }
            "installation-path" => {
                let input_value = value.get().expect("The value need to be of type `String`");
                self.installation_path.replace(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name().to_value(),
            "icon" => self.icon.borrow().to_value(),
            "package-name" => self.package_name.borrow().to_value(),
            "path" => self.path.borrow().to_value(),
            "installation-path" => self.installation_path.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
