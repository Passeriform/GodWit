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
}

custom_error! {pub EnvError
	IO {source: std::io::Error} = "A resource IO error occured while operating environment variables.",
}

custom_error! {pub TuiError
	IO {source: std::io::Error} = "A resource IO error occured while opera.",
	Crossterm {source: crossterm::ErrorKind} = "Crossterm threw unexpected error.",
	MpscRecv {source: std::sync::mpsc::RecvError} = "Unexpected Mpsc reveive error occured.",
}

custom_error! {pub SettingsError
	WorkingDirNotFound = "The Godwit working directory was not found.",
	SettingsNotFound{file: String} = "The settings {file} was not found.",

	SettingsAlreadyExists{mode: SettingsMode} = "The settings of type {mode} already exist.",

	InvalidSettings{file: String} = "The settings {file} seems to be invalid.",

	DisallowedUpsert = "Disallowd because upsert option wasn't passed.",
	DisallowedHeadless = "Disallowed because the operation isn't permitted on headless usage.",

	IO {source: std::io::Error} = "IO operations failed unexpectedly.",
	Pattern {source: glob::PatternError} = "Glob pattern matching threw an error.",
}

custom_error! {pub StateError
	StateGraphNotFound{file: String} = "The state-graph {file} was not found.",
	StateNotFound{state: String} = "The state {state} was not found.",

	StateGraphAlreadyExists = "A state-graph already exists.",
	StateAlreadyExists{state: String} = "The state {state} already exists.",

	InvalidStateGraph{file: String} = "The state-graph {file} seems to be invalid.",
	InvalidState{state: String} = "The state {state} seems to be invalid.",

	EmptyStateList = "No states were found.",

	IO {source: std::io::Error} = "IO operations failed unexpectedly.",
	Settings {source: SettingsError} = "Unexpected error thrown in Settings module.",
	Env {source: EnvError} = "Unexpected error thrown in Env module.",
}

custom_error! {pub IOError
	IO {source: std::io::Error} = "IO operations failed unexpectedly.",
	State {source: StateError} = "State couldn't be fetched in IO module.",
}

custom_error! {pub PluginError
	StdErr {message: String} = "Child detached process returned error {message}.",
	PluginNotFound {plugin: String} = "The plugin {plugin} was not found.",

	IO {source: std::io::Error} = "IO operations failed unexpectedly.",
	FromUtf8 {source: std::string::FromUtf8Error} = "UTF-8 conversion threw an error.",

	Settings {source: SettingsError} = "Setup failed due to error thrown by Settings module.",
}

custom_error! {pub SetupError
	Settings {source: SettingsError} = "Unexpected error thrown in Settings module.",
	State {source: StateError} = "Unexpected error thrown in State module.",
}

custom_error! {pub CoreError
	State {source: StateError} = "Unexpected error thrown in State module.",
	Setup {source: SetupError} = "Unexpected error thrown in setting up Godwit.",
	Plugin {source: PluginError} = "Unexpected error thrown in State module.",
	Tui {source: TuiError} = "Unexpected error thrown in State module.",
}
