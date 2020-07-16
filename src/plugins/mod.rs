//! Plugin Interface
//!
//! A composite processor for all godwit-compatible plugins.
//! Must follow a unified standard to keep minimal deviation.
use crate::errors::{IOError, PluginError};
use crate::settings;
use getter_derive::Getter;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::{Command, Output, Stdio};

// TODO: Convert this into a trait

/// Plugin structure
#[derive(Clone, Debug, Deserialize, Serialize, Getter)]
#[serde(rename_all = "snake_case")]
pub struct Plugin {
	name: String,
	exec: String,
}

impl Default for Plugin {
	fn default() -> Self {
		Plugin {
			name: Default::default(),
			exec: Default::default(),
		}
	}
}

/// Call a detached external process for a plugin.
pub fn invoke(plugin_str: &str, args_options: Option<Vec<&str>>) -> Result<Output, Box<dyn Error>> {
	let command_str = get_plugin(plugin_str)?.get_exec();
	let mut args = args_options.unwrap_or_default();

	let command = str_to_command!(command_str, &mut args);

	let output = Command::new(command).args(args).output()?;

	Ok(output)
}

/// Bind stdio to spawned child process and pipe io to it.
pub fn bind(plugin_str: &str, args_options: Option<Vec<&str>>) -> Result<Vec<u8>, Box<dyn Error>> {
	let command_str = get_plugin(plugin_str)?.get_exec();
	let mut args = args_options.unwrap_or_default();

	let command = str_to_command!(command_str, &mut args);

	let child = Command::new(command)
		.args(args)
		.stdin(Stdio::piped())
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	let output = child.wait_with_output()?;

	if output.status.success() {
		Ok(output.stdout)
	} else {
		Err(IOError::StdErr {
			err: String::from_utf8(output.stderr)?,
		}
		.into())
	}
}

/// Retrieve plugin from queried name.
pub fn get_plugin(q_plugin: &str) -> Result<Plugin, Box<dyn Error>> {
	let plugins = settings::get_settings()?.get_plugins();
	plugins
		.into_iter()
		.find(|plugin| plugin.name == q_plugin)
		.map_or(
			Err(PluginError::PluginNotFound {
				plugin: q_plugin.into(),
			}
			.into()),
			|plugin| Ok(plugin),
		)
}

/// Create new plugin.
pub fn new(name: &str, exec: &str) -> Plugin {
	Plugin {
		name: name.to_string(),
		exec: exec.to_string(),
	}
}
