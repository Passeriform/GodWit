//! Templated Setup Core
//!
//! Separated setup core chunk for ease of abstraction. Must be percieved as one
//! unit with the core. It contains additional API calls for templated
//! setups.

use crate::errors::{SettingsError, SettingsMode, SetupError, StateError};
use crate::settings::{self, Settings};
use crate::statehandler::{self, StateGraph};
use log::info;
use std::path::PathBuf;

/// Setup working directory and initialize settings.
pub fn setup_gw_dir(
	cfgdir: Option<PathBuf>,
	headless: bool,
	refresh: bool,
) -> Result<(), SetupError> {
	if refresh {
		settings::purge_settings(true)?
	}

	settings::get_settings()
		.and_then(|settings| {
			if settings.get_headless() {
				return Err(SettingsError::SettingsAlreadyExists {
					mode: SettingsMode::Headless,
				});
			} else {
				return Err(SettingsError::SettingsAlreadyExists {
					mode: SettingsMode::Full,
				});
			}
		})
		.or_else(|_| {
			info!("Creating working directory at {:?}", cfgdir);
			Settings::init(cfgdir, None, headless, true, None).save_settings(true)
		})
		.map_err(Into::into)
}

/// Setup state files and initialize state-graph.
pub fn setup_init_state() -> Result<(), SetupError> {
	if statehandler::load_stategraph().is_ok() {
		info!("Found an existing stategraph. Pass killsave option to overwrite.");
		return Err(StateError::StateGraphAlreadyExists.into());
	}

	info!("Creating new stategraph...");

	StateGraph::default()
		.propagate()
		.and_then(|_| {
			info!(
				"Your new active save is at {:?}.",
				settings::get_settings()?.get_save_state()?.display()
			);
			Ok(())
		})
		.map_err(Into::into)
}
