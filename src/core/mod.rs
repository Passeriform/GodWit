/// Templated Setup Core
///
/// Separated setup core chunk for ease of abstraction. Must be percieved as one
/// unit with the core. It contains additional API calls for templated
/// setups.
mod setup;
/// TUI Splash Core
///
/// TUI splash procedure core. Dictates the TUI windows and operations.
pub mod splash;

use crate::{
	core::setup::{setup_gw_dir, setup_init_state},
	glyph::Glyph,
	plugins,
	statehandler::{self, State},
};
use std::{error::Error, io, path::PathBuf};

/// One-time Godwit setup call.
pub fn init(path: Option<PathBuf>, headless: bool, refresh: bool) -> Result<(), Box<dyn Error>> {
	setup_gw_dir(path, headless, refresh)?;
	setup_init_state()?;
	Ok(())
}

/// Add project to Godwit.
pub fn add(
	glyph: Glyph,
	location: PathBuf,
	existing: bool,
	active: bool,
	default: bool,
) -> Result<(), Box<dyn Error>> {
	if !existing {
		plugins::invoke("Weaver", None)?;
	}

	statehandler::add_state(glyph, location, None, active, default)?;
	Ok(())
}

/// Remove project from Godwit
pub fn remove(glyph: Glyph) -> Result<(), Box<dyn Error>> {
	statehandler::purge_state(glyph)?;
	Ok(())
}

/// List projects under Godwit.
pub fn list() -> Result<Vec<State>, Box<dyn Error>> {
	let states = statehandler::load_stategraph()?.get_states().to_vec();
	Ok(states)
}

/// Switch to another project under Godwit.
pub fn switch(glyph: Glyph, default: bool) -> Result<(), Box<dyn Error>> {
	statehandler::set_active(&glyph)?;

	if default {
		statehandler::set_default(&glyph)?;
	}

	Ok(())
}

/// Forward to splash TUI.
pub fn runsplash() -> Result<(), io::Error> {
	splash::run()?;
	Ok(())
}
