//! Godwit Core
//!
//! General abstraction over the operations used as an API with call
//! forward wrappers. It contains full API definitions and endpoints to
//! integrate Godwit core.
mod setup;

use crate::{
	core::setup::{setup_gw_dir, setup_init_state},
	glyph::Glyph,
	plugins,
	statehandler::{self, State},
	tui,
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
	tui::run()?;
	Ok(())
}
