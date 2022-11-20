mod imp;

use adw::prelude::*;
use gtk::{
    gio,
    glib::{self, clone, wrapper},
    prelude::SettingsExt,
    subclass::prelude::*,
    traits::{DialogExt, GtkWindowExt, NativeDialogExt, WidgetExt},
};

use crate::{
    launcher::{LauncherData, LauncherObject},
    APP_ID,
};

wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn setup_settings(&self) {
        let settings = gio::Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`")
    }

    fn settings(&self) -> &gio::Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let size = self.default_size();
        let settings = self.settings();

        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;
        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = self.settings();
        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    fn setup_actions(&self) {
        let action_new_launcher = gio::SimpleAction::new("add_new_launcher", None);
        action_new_launcher.connect_activate(clone!(@weak self as window => move |_,_| {
            window.add_new_launcher();
        }));

        self.add_action(&action_new_launcher);
    }

    fn setup_callbacks(&self) {
        self.imp()
            .back_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.imp().leaflet.navigate(adw::NavigationDirection::Back);
            }));

        self.imp().launcher_list.connect_row_activated(
            clone!(@weak self as window => move |_, row| {
                let index = row.index();
                let selected_launcher = window
                    .launchers()
                    .item(index as u32)
                    .expect("There needs to be an object at this position")
                    .downcast::<LauncherObject>()
                    .expect("The object needs to be a `LauncherObject`");

                window.set_current_launcher(selected_launcher);
                window.imp().leaflet.navigate(adw::NavigationDirection::Forward);
            }),
        );
    }

    fn setup_launchers(&self) {
        let launchers = gio::ListStore::new(LauncherObject::static_type());
        self.imp()
            .launchers
            .set(launchers.clone())
            .expect("Could not set launchers");

        self.imp().launcher_list.bind_model(
            Some(&launchers),
            clone!(@weak self as window => @default-panic, move |obj| {
                let launcher_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `LauncherObject`");

                let row = window.create_launcher_row(launcher_object);
                row.upcast()
            }),
        )
    }

    fn get_datas(&self) {
        let launchers: Vec<LauncherObject> = LauncherData::find_all_launchers()
            .into_iter()
            .map(LauncherObject::from_launcher_data)
            .collect();

        self.launchers().extend_from_slice(&launchers);
    }

    fn create_launcher_row(&self, launcher_object: &LauncherObject) -> gtk::ListBoxRow {
        let icon = gtk::Image::builder()
            .resource(&launcher_object.imp().icon.borrow())
            .icon_size(gtk::IconSize::Large)
            .build();

        let label = gtk::Label::builder()
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .xalign(0.0f32)
            .margin_start(12)
            .build();

        launcher_object
            .bind_property("name", &label, "label")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        let hori_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        hori_box.append(&icon);
        hori_box.append(&label);
        hori_box.set_tooltip_text(Some(&launcher_object.imp().name()));

        gtk::ListBoxRow::builder().child(&hori_box).build()
    }

    fn _current_launcher(&self) -> LauncherObject {
        self.imp()
            .current_launcher
            .borrow()
            .clone()
            .expect("`current_launcher` should be set in `set_current_launcher`")
    }

    fn launchers(&self) -> gio::ListStore {
        self.imp()
            .launchers
            .get()
            .expect("`launchers` should be set in `setup_launchers`")
            .clone()
    }

    fn set_current_launcher(&self, launcher: LauncherObject) {
        self.imp().current_launcher.replace(Some(launcher));
    }

    fn add_new_launcher(&self) {
        let dialog = gtk::Dialog::with_buttons(
            Some("Add custom launcher"),
            Some(self),
            gtk::DialogFlags::MODAL
                | gtk::DialogFlags::DESTROY_WITH_PARENT
                | gtk::DialogFlags::USE_HEADER_BAR,
            &[
                ("Cancel", gtk::ResponseType::Cancel),
                ("Add", gtk::ResponseType::Accept),
            ],
        );
        dialog.set_default_response(gtk::ResponseType::Accept);

        let dialog_button = dialog
            .widget_for_response(gtk::ResponseType::Accept)
            .expect("");
        dialog_button.set_sensitive(false);

        let folder_choose_button = gtk::Button::builder()
            .label("Choose folder")
            .margin_end(12)
            .margin_top(12)
            .margin_start(12)
            .margin_bottom(4)
            .build();
        folder_choose_button.connect_clicked(
            clone!(@weak dialog, @weak folder_choose_button, @weak self as window => move |_| {
            let folder_choose = gtk::FileChooserNative::new(
                Some("Choose custom launcher folder"),
                Some(&dialog),
                gtk::FileChooserAction::SelectFolder,
                Some("Choose"),
                Some("Cancel")
            );

            folder_choose.connect_response(
                clone!(@strong folder_choose, @weak folder_choose_button, @weak window => move |info, response| {
                    if response != gtk::ResponseType::Accept {
                        return;
                    }

                    println!("Files: {:?}", info);
                    folder_choose_button.set_tooltip_text(Some("Hello World"));
                }),
            );
            folder_choose.show();
            }),
        );

        let launcher_type = gtk::DropDown::builder()
            .model(&gtk::StringList::new(&["Steam", "Lutris"]))
            // .enable_search(true)
            .margin_end(12)
            .margin_top(4)
            .margin_start(12)
            .margin_bottom(12)
            .build();

        dialog.content_area().append(&folder_choose_button);
        dialog.content_area().append(&launcher_type);
        dialog.connect_response(
            clone!(@weak self as window, @weak folder_choose_button => move |dialog, response| {
                dialog.destroy();

                if response != gtk::ResponseType::Accept {
                    return;
                }
            }),
        );

        dialog.show();
    }
}
