#[derive(Default)]
pub struct UltralightSettings {
    loadShadersFromFileSystem: Option<bool>,

    fileSystemPath: Option<String>,
}

impl UltralightSettings {
    pub fn new() -> UltralightSettings {
        UltralightSettings {
            loadShadersFromFileSystem: None,

            fileSystemPath: None,
        }
    }

    pub fn to_ulsettings(&self) -> ul_sys::ULSettings {
        let settings = unsafe {
            ul_sys::ulCreateSettings()
        };

        set_config!(settings, self, loadShadersFromFileSystem, ulSettingsSetLoadShadersFromFileSystem);

        set_config_str!(settings, self, fileSystemPath, ulSettingsSetFileSystemPath);

        settings
    }

    config_item!( loadShadersFromFileSystem, bool, "Set whether or not we should load and compile shaders from the file system (eg, from the /shaders/ path, relative to file_system_path)." );

    config_item!( fileSystemPath, String, "Set the root file path for our file system, you should set this to the relative path where all of your app data is." );
}
