/// Templated Setup Core
///
/// Separated setup core chunk for ease of abstraction. Must be percieved as one unit with the core.
/// It contains additionally exposed API calls for templated setups.
///
/// Functions:
///    setup_gw_dir -> Setup Godwit working directory and perform initial settings loadup.
///    setup_init_state -> Setup inital working state for state-graph bootstrapping.

use std::{
    fs,
    path::PathBuf,
    // io::{stdin, Read}
};
use crate::settings::{
    fetch,
    Settings
};

pub fn setup_gw_dir(cfgdir: Option<PathBuf>, headless: Option<bool>) -> Result<(), &'static str> {
    match fetch::get_settings() {
        Ok(fetch_settings) => {
            if fetch_settings.headless {
                println!("Found a headless usage with a .godwitrc. States will be flushed on end of command life.");
                println!("No need for init on a headless usage. Shift .godwitrc to .godwit/settings.gwcore");
            }
            else {
                println!("Found a settings directory.");
            }
        },
        Err(_) => {
            println!("No existing .godwitrc or .godwit directory found");

            // let option: Option<char> = stdin().bytes()
            // .next()
            // .and_then(|result| result.ok())
            // .map(|byte| byte as char)
            // .filter(|c| c.is_alphabetic())
            // .map(|mchar| mchar.to_lowercase().next().unwrap());
            //
            // match option {
            //     Some('y')   => {
            //
            //     }
            //     Some('n')   => {
            //         println!("Skipping directory creation. Running headless.");
            //     }
            //     Some(other)  => {
            //         println!("Got unknown symbol '{}'. Assuming response 'n'. Config directory won't be created. Running headless.", other);
            //     }
            //     None    => {
            //         println!("Got no symbol. Assuming response 'n'. Config directory won't be created. Running headless.");
            //     }
            // }

            let working_dir = [cfgdir.unwrap_or(dirs::home_dir().expect("Home couldn't be located in current $PATH variables.")), PathBuf::from(".godwit")].iter().collect::<PathBuf>();
            println!("Creating working directory at {:?}", working_dir.display());

            fs::create_dir_all(&working_dir).expect("Error occured while creating directories.");

            let upsert = Some(true);
            if let Err(e) = Settings::init(Some(working_dir), None, headless, None).save_settings(upsert) {
                return Err(e)
            };
        }
    };

    Ok(())
}

pub fn setup_init_state() -> Result<(), &'static str> {
    fs::create_dir_all(fetch::get_settings()?.get_states_dir()).expect("State directory doesn't exist and can't be created.");
    fs::copy("assets/templates/default.gwsg", fetch::get_settings()?.get_save_state()).expect("Error occured while copying templates.");

    println!("Your new active save is at {:?}.", fetch::get_settings()?.get_save_state().display());
    Ok(())
}
