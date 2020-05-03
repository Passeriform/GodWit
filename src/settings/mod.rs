/// Godwit Settings Management
///
/// A base utility abstraction over persistent settings and access methods.
///
/// Modules:
///    fetch -> Exposes attribute fetching methods.
///    edit -> Exposes alteration methods.
///
/// Structs/Enums:
///    Settings -> Define structure for Godwit settings.
///
/// Implementation methods:
///    init -> Prepares new settings file and working directory.
///    get_working_dir -> Returns working directory location.
///    get_states_dir -> Returns save-states directory location.
///    get_save_state -> Returns active/alternate save-state path.
///
/// Functions:
///    get_settings -> Returns a current verified settings struct.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::plugins::Plugin;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Settings {
    pub working_dir: PathBuf,
    pub states_dir: PathBuf,
    pub headless: bool,
    pub plugins: Vec<Plugin>,
}

impl Settings {
    pub fn init(working_dir: Option<PathBuf>, states_dir: Option<PathBuf>, headless: Option<bool>, plugins: Option<Vec<Plugin>>) -> Self {
        let home_dir = PathBuf::from(dirs::home_dir().expect("Home couldn't be located in current $PATH variables."));

        let working_dir = working_dir.unwrap_or([
            &home_dir,
            &PathBuf::from(".godwit")
        ].iter().collect());

        let states_dir = states_dir.unwrap_or([
            &working_dir,
            &PathBuf::from("states")
        ].iter().collect());

        let headless = headless.unwrap_or(false);

        let plugins = plugins.unwrap_or(vec!(Plugin::default()));

        Settings {
            working_dir: working_dir,
            states_dir: states_dir,
            headless: headless,
            plugins: plugins,
        }
    }
}

impl Default for Settings {
  fn default() -> Self {
    let home_dir = PathBuf::from(dirs::home_dir().expect("Home couldn't be located in current $PATH variables."));

    let working_dir = [
        &home_dir,
        &PathBuf::from("Appdata/local/godwit") // Make unified for all platforms
    ].iter().collect();

    let states_dir: PathBuf = [
        &working_dir,
        &PathBuf::from("states")
    ].iter().collect();

    let plugins = vec!(Plugin::default());

    Settings {
        working_dir: working_dir,
        states_dir: states_dir,
        headless: true,
        plugins: plugins,
    }
  }
}

pub mod fetch {
    use glob::glob;
    use std::path::PathBuf;
    use std::fs::File;
    use crate::plugins::Plugin;
    use super::Settings;


    impl Settings {
        pub fn get_working_dir(self) -> PathBuf {
            return self.working_dir;
        }

        pub fn get_states_dir(self) -> PathBuf {
            return self.states_dir;
        }

        pub fn get_save_state(self) -> PathBuf {
            let mut save_state_path = self.states_dir;

            for save_file in glob(&(save_state_path.to_string_lossy().into_owned() + "/*.gwsg")).expect("Failed to read glob pattern.") {
                return save_file.unwrap();
            }

            save_state_path.push("active.gwsg");
            return save_state_path;
        }

        pub fn get_plugins(self) -> Vec<Plugin> {
            return self.plugins;
        }

        pub fn save_settings(self, upsert: Option<bool>) -> Result<(), &'static str> {
            let home_dir = PathBuf::from(dirs::home_dir().expect("Home couldn't be located in current $PATH variables."));

            let working_dir: PathBuf = [&home_dir, &PathBuf::from(".godwit")].iter().collect();

            let rc_path: PathBuf = [&home_dir, &PathBuf::from(".godwitrc")].iter().collect();

            let settings_path: PathBuf = [&working_dir, &PathBuf::from("settings.gwcore")].iter().collect();

            if rc_path.exists() {
                let settings_file = File::open(rc_path).expect("File couldn't be opened");
                serde_json::to_writer_pretty(settings_file, &self).expect("State JSON is invalid");

                Ok(())
            }
            else if settings_path.exists() {
                let settings_file = File::open(settings_path).expect("File couldn't be opened");
                serde_json::to_writer_pretty(settings_file, &self).expect("State JSON is invalid");

                Ok(())
            }
            else {
                if let Some(true) = upsert {
                    let settings_file = File::create(settings_path).expect("File couldn't be opened");
                    serde_json::to_writer_pretty(settings_file, &self).expect("State JSON is invalid");

                    Ok(())
                }
                else {
                    Err("Settings file missing. Upsert not allowed.")
                }
            }
        }
    }

    pub fn get_settings() -> Result<Settings, &'static str> {
        let home_dir = PathBuf::from(dirs::home_dir().expect("Home couldn't be located in current $PATH variables."));

        let working_dir: PathBuf = [&home_dir, &PathBuf::from(".godwit")].iter().collect();

        let rc_path: PathBuf = [&home_dir, &PathBuf::from(".godwitrc")].iter().collect();

        let settings_path: PathBuf = [&working_dir, &PathBuf::from("settings.gwcore")].iter().collect();

        if rc_path.exists() {
            println!("Found local .godwitrc at {}", rc_path.display());
            let settings_file = File::open(rc_path).expect("File couldn't be opened");
            let settings: Settings = serde_json::from_reader(settings_file).expect("State JSON is invalid");

            Ok(settings)
        }
        else if settings_path.exists() {
            println!("Found settings file at {}", settings_path.display());
            let settings_file = File::open(settings_path).expect("File couldn't be opened");
            let settings: Settings = serde_json::from_reader(settings_file).expect("State JSON is invalid");

            Ok(settings)
        }
        else if working_dir.exists() {
            println!("Found no generic settings files. Although the working directory is present.");
            Err("No settings file found. Maybe corrupted installation.")
        }
        else {
            println!("No settings file found. Working directory uninitialized.");
            Err("No working directory found.")
        }
    }
}

pub mod edit { }
