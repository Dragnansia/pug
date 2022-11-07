mod imp;

use crate::APP_ID;
use adw::prelude::*;
use gtk::{
    gio,
    glib::{self, clone, wrapper},
    prelude::SettingsExt,
    subclass::prelude::*,
    traits::{DialogExt, GtkWindowExt, NativeDialogExt, WidgetExt},
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
            .enable_search(true)
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
