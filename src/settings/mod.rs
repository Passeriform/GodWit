//! Godwit Settings Management
//!
//! A utility abstraction over persistent settings and access methods.
use crate::errors::SettingsError;
use crate::plugins::Plugin;
use getter_derive_rs::Getter;
use glob::glob;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
	error::Error,
	fs::{self, File},
	path::PathBuf,
};

/// Define godwit settings.
#[derive(Debug, Deserialize, Serialize, Getter)]
#[serde(rename_all = "snake_case")]
pub struct Settings {
	pub working_dir: Option<PathBuf>,
	pub states_dir: Option<PathBuf>,
	pub headless: bool,
	pub switch_on_add: bool,
	pub plugins: Vec<Plugin>,
}

impl Settings {
	/// Returns new settings instance.
	pub fn init(
		working_dir: Option<PathBuf>,
		states_dir: Option<PathBuf>,
		headless: bool,
		switch_on_add: bool,
		plugins: Option<Vec<Plugin>>,
	) -> Self {
		let home_dir = PathBuf::from(
			dirs::home_dir().expect("Home couldn't be located in current $PATH variables."),
		);
		let plugins = plugins.unwrap_or(Default::default());

		let _working_dir;
		let _states_dir;

		if headless {
			_working_dir = Some(home_dir);
			_states_dir = None;
		} else {
			_working_dir = working_dir.map_or_else(
				|| Settings::default().working_dir,
				|working_dir| Some(working_dir),
			);

			_states_dir = states_dir.map_or_else(
				|| Settings::default().states_dir,
				|states_dir| Some(states_dir),
			);
		}

		Settings {
			working_dir: _working_dir,
			states_dir: _states_dir,
			headless: headless,
			switch_on_add: switch_on_add,
			plugins: plugins,
		}
	}

	/// Returns state-graph source file.
	pub fn get_save_state(&self) -> Result<PathBuf, Box<dyn Error>> {
		if self.headless {
			return Err(SettingsError::DisallowedHeadless.into());
		}

		let mut save_state_path = self.states_dir.clone().unwrap_or_default();

		for state_file_path in glob(&(save_state_path.to_string_lossy().into_owned() + "/*.gwsg"))?
		{
			return Ok(state_file_path.unwrap_or_else(|_| {
				[&save_state_path, &PathBuf::from("active.gwsg")]
					.iter()
					.collect()
			}));
		}

		save_state_path.push("active.gwsg");
		Ok(save_state_path)
	}

	/// Propagates changes to settings.
	pub fn save_settings(&self, upsert: bool) -> Result<(), Box<dyn Error>> {
		let working_dir = &self.working_dir.clone().unwrap(); // This should always give panic on None

		if !working_dir.exists() {
			fs::create_dir_all(working_dir)?;
		}

		let settings_path: PathBuf;

		if self.headless {
			settings_path = [&working_dir, &PathBuf::from(".gwrc")].iter().collect();

			if !upsert {
				return Err(SettingsError::DisallowedUpsert.into());
			}
		} else {
			let states_dir = &self.states_dir.clone().unwrap_or_default();
			settings_path = [&working_dir, &PathBuf::from("settings.gwcore")]
				.iter()
				.collect();

			if !states_dir.exists() {
				fs::create_dir_all(states_dir)?;
			}

			if !upsert {
				return Err(SettingsError::DisallowedUpsert.into());
			}
		}

		File::create(settings_path).and_then(|settings_file| {
			serde_json::to_writer_pretty(settings_file, &self)?;
			Ok(())
		})?;
		Ok(())
	}
}

impl Default for Settings {
	fn default() -> Self {
		let home_dir = PathBuf::from(
			dirs::home_dir().expect("Home couldn't be located in current $PATH variables."),
		);

		let working_dir = [&home_dir, &PathBuf::from(".godwit")].iter().collect();

		let states_dir: PathBuf = [&working_dir, &PathBuf::from("states")].iter().collect();

		let plugins = Default::default();

		Settings {
			working_dir: Some(working_dir),
			states_dir: Some(states_dir),
			headless: false,
			switch_on_add: true,
			plugins: plugins,
		}
	}
}

/// Get settings instance from settings source file.
pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
	let home_dir = PathBuf::from(
		dirs::home_dir().expect("Home couldn't be located in current $PATH variables."),
	);

	let working_dir: PathBuf = [&home_dir, &PathBuf::from(".godwit")].iter().collect();

	let rc_path: PathBuf = [&home_dir, &PathBuf::from(".gwrc")].iter().collect();

	let settings_path: PathBuf = [&working_dir, &PathBuf::from("settings.gwcore")]
		.iter()
		.collect();

	if rc_path.exists() {
		info!("Godwitrc found at {}", rc_path.display());
		let settings = File::open(rc_path).and_then(|settings_file| {
			let settings: Settings = serde_json::from_reader(settings_file)?;
			Ok(settings)
		})?;
		Ok(settings)
	} else if settings_path.exists() {
		info!("Settings core found at {}", settings_path.display());
		let settings = File::open(settings_path).and_then(|settings_file| {
			let settings: Settings = serde_json::from_reader(settings_file)?;
			Ok(settings)
		})?;
		Ok(settings)
	} else if working_dir.exists() {
		info!(
			"No settings files found. Working directory exists at {}.",
			working_dir.display()
		);
		Err(SettingsError::SettingsNotFound {
			file: settings_path.to_string_lossy().into_owned(),
		}
		.into())
	} else {
		info!("Working directory doesn't exist.");
		Err(SettingsError::SettingsNotFound {
			file: working_dir.to_string_lossy().into_owned(),
		}
		.into())
	}
}

/// Purges settings source file and states.
pub fn purge_settings(purge_states: bool) -> Result<(), Box<dyn Error>> {
	let home_dir = PathBuf::from(
		dirs::home_dir().expect("Home couldn't be located in current $PATH variables."),
	);

	let working_dir: PathBuf = [&home_dir, &PathBuf::from(".godwit")].iter().collect();

	let rc_path: PathBuf = [&home_dir, &PathBuf::from(".gwrc")].iter().collect();

	let settings_path: PathBuf = [&working_dir, &PathBuf::from("settings.gwcore")]
		.iter()
		.collect();

	if rc_path.exists() {
		info!("Purging {}", rc_path.display());
		fs::remove_file(rc_path)?;
	} else if working_dir.exists() {
		if purge_states {
			info!("Purging {}.", working_dir.display());
			fs::remove_dir_all(working_dir)?;
		} else {
			info!("Purging {}.", settings_path.display());
			fs::remove_file(settings_path)?;
		}
	} else {
		return Err(SettingsError::WorkingDirNotFound.into());
	}
	Ok(())
}
