mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct LauncherObject(ObjectSubclass<imp::LauncherObject>);
}

impl LauncherObject {
    pub fn new(infos: &LauncherInfos, path: String, installation_path: String) -> Self {
        glib::Object::builder()
            .property("name", &infos.name)
            .property("icon", &infos.icon)
            .property("package-name", &infos.package_name)
            .property("path", &path)
            .property("installation-path", &installation_path)
            .build()
    }

    pub fn from_launcher_data(data: LauncherData) -> Self {
        Self::new(&data.infos, data.path, data.installation_path)
    }
}

#[derive(Default, Clone)]
pub struct LauncherData {
    pub infos: LauncherInfos,
    pub path: String,
    pub installation_path: String,
}

impl LauncherData {
    fn read_datas_files() -> Vec<Self> {
        vec![]
    }

    pub fn find_all_launchers() -> Vec<Self> {
        Self::read_datas_files()
    }
}

#[derive(Clone)]
pub struct LauncherInfos {
    pub name: String,
    /// Icon gresource path
    pub icon: String,

    /// To display custom package manager (flatpak, snapd, ...)
    pub package_name: Option<String>,
}

impl Default for LauncherInfos {
    fn default() -> Self {
        Self::steam(None)
    }
}

impl LauncherInfos {
    pub fn steam(package_name: Option<String>) -> Self {
        Self {
            name: "Steam".into(),
            icon: "/org/pug/images/steam.png".into(),
            package_name,
        }
    }

    pub fn lutris(package_name: Option<String>) -> Self {
        Self {
            name: "Lutris".into(),
            icon: "/org/pug/images/lutris.png".into(),
            package_name,
        }
    }

    pub fn heroic(compat_name: &str, package_name: Option<String>) -> Self {
        Self {
            name: format!("Heroic Game Launcher ({compat_name})"),
            icon: "/org/pug/images/heroic.png".into(),
            package_name,
        }
    }
}
