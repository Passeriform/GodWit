/// Plugin Interface
///
/// Will be used as a unified processor for all godwit-compatible plugins.
/// Must follow a unified standard to keep minimal branching.
///
/// Functions:
///    invoke -> One time godwit working setup call with settings return
///    pipe -> add project to godwit tracker
///    query -> list tracked projects

use std::{
    process::{
        Output,
        Command,
        Stdio
    }
};
use serde::{Deserialize, Serialize};
use crate::settings::fetch;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Plugin {
    name: String,
    exec: String,
}

impl Default for Plugin {
  fn default() -> Self {
    Plugin {
        name: "Sanity".to_string(),
        exec: "yes".to_string(),
    }
  }
}

impl Plugin {
    pub fn get_exec(self) -> String {
        return self.exec;
    }
}

pub fn invoke(plugin_str: &str, args: Option<Vec<&str>>) -> Result<Output, &'static str> {
    let output = Command::new(get_plugin(plugin_str.to_string())?.get_exec())
    .args(args.unwrap_or(Vec::new()))
    .output()
    .expect("Process failed to return.");

    Ok(output)
}

pub fn pipe(plugin_str: &str, args: Option<Vec<&str>>) -> Result<Vec<u8>, Vec<u8>> {
    let child = Command::new(get_plugin(plugin_str.to_string())?.get_exec())
        .args(args.unwrap_or(Vec::new()))
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Process failed to return.");

    let output = child
        .wait_with_output()
        .expect("Process failed on wait");

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(output.stderr)
    }
}

pub fn query() -> Result<(), &'static str> {

    Ok(())
}

pub fn get_plugin(name: String) -> Result<Plugin, &'static str> {
    let plugins = fetch::get_settings()?.get_plugins();
    Ok(plugins.into_iter().find(|plugin| plugin.name == name).unwrap_or(Plugin::default()))
}

pub fn new(name: Option<&str>, exec: Option<&str>) -> Plugin {
    return Plugin {
        name: String::from(name.unwrap_or_default()),
        exec: String::from(exec.unwrap_or_default())
    }
}
