//! Godwit Core
//!
//! General abstraction over the operations used as an API with call
//! forward wrappers. It contains full API definitions and endpoints to
//! integrate Godwit core.
mod setup;

use crate::core::setup::{setup_gw_dir, setup_init_state};
use crate::errors::CoreError;
use crate::glyph::Glyph;
use crate::plugins;
use crate::statehandler::{self, State};
use crate::tui;
use std::path::PathBuf;

/// One-time Godwit setup call.
pub fn init(path: Option<PathBuf>, headless: bool, refresh: bool) -> Result<(), CoreError> {
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
) -> Result<(), CoreError> {
	if !existing {
		plugins::invoke("Weaver", None)?;
	}

	statehandler::add_state(glyph, location, None, active, default)?;
	Ok(())
}

/// Remove project from Godwit
pub fn remove(glyph: Glyph) -> Result<(), CoreError> {
	statehandler::purge_state(glyph)?;
	Ok(())
}

/// List projects under Godwit.
pub fn list() -> Result<Vec<State>, CoreError> {
	let states = statehandler::load_stategraph()?.get_states().to_vec();
	Ok(states)
}

/// Switch to another project under Godwit.
pub fn switch(glyph: Glyph, default: bool) -> Result<(), CoreError> {
	statehandler::set_active(glyph.clone())?;

	if default {
		statehandler::set_default(glyph.clone())?;
	}

	Ok(())
}

/// Forward to splash TUI.
pub fn runsplash() -> Result<(), CoreError> {
	tui::run()?;
	Ok(())
}
