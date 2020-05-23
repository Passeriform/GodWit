//! Errors
//!
//! Errors for Godwit suite and services.
use custom_error::custom_error;
use std::fmt;

#[derive(Debug)]
pub enum SettingsMode {
	Headless,
	Full,
}

impl fmt::Display for SettingsMode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SettingsMode :: {:?}", self)
	}
}

custom_error! {pub GlyphError
	InvalidGlyph{glyph: String} = "The glyph {glyph} doesn't seem to be a valid glyph.",
	Unexpected                  = "Unexpected token"
}
custom_error! {pub IOError
	StdErr{err: String} = "{err}",
}
custom_error! {pub StateError
	StateGraphNotFound{file: String} = "The state-graph {file} was not found.",
	StateNotFound{state: String} = "The state {state} was not found.",

	StateGraphAlreadyExists = "A state-graph already exists.",
	StateAlreadyExists{state: String} = "The state {state} already exists.",

	InvalidStateGraph{file: String} = "The state-graph {file} seems to be invalid.",
	InvalidState{state: String} = "The state {state} seems to be invalid.",

	EmptyStateList = "No states were found.",

}
custom_error! {pub PluginError
	PluginNotFound{plugin: String} = "The plugin {plugin} was not found.",
}

custom_error! {pub SettingsError
	WorkingDirNotFound = "The Godwit working directory was not found.",
	SettingsNotFound{file: String} = "The settings {file} was not found.",

	SettingsAlreadyExists{mode: SettingsMode} = "The settings of type {mode} already exist.",

	InvalidSettings{file: String} = "The settings {file} seems to be invalid.",

	DisallowedUpsert = "Disallowd because upsert option wasn't passed.",
	DisallowedHeadless = "Disallowed because the operation isn't permitted on headless usage."
}
