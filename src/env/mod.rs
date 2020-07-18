//! Environment Manipulation Utility
//!
//! A utility abstraction over Godwit environment variables.
use crate::errors::EnvError;
use dirs;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::IntoIterator;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Copy, Clone, Debug)]
pub enum Shell {
	BASH,
	ZSH,
	FISH,
	KSH,
	CSH,
	TCSH,
}

impl Shell {
	pub fn iterator() -> impl Iterator<Item = Shell> {
		[
			Shell::BASH,
			Shell::ZSH,
			Shell::FISH,
			Shell::KSH,
			Shell::CSH,
			Shell::TCSH,
		]
		.iter()
		.copied()
	}
}

// Shamelessly borrowed from shellexpand (https://github.com/netvl/shellexpand/blob/master/src/lib.rs)
pub fn fix_tilde<S: ?Sized>(input: &S) -> std::borrow::Cow<str>
where
	S: AsRef<str>,
{
	let input_str = input.as_ref();
	if input_str.starts_with("~") {
		let input_after_tilde = &input_str[1..];
		if input_after_tilde.is_empty() || input_after_tilde.starts_with("/") {
			let result = format!(
				"{}{}",
				dirs::home_dir().unwrap().display(),
				input_after_tilde
			);
			result.into()
		} else {
			input_str.into()
		}
	} else {
		input_str.into()
	}
}

pub fn check_paths_exist<'a, T>(list: T) -> Vec<PathBuf>
where
	T: IntoIterator<Item = &'a str>,
{
	list.into_iter()
		.map(|raw_path_str| fix_tilde(raw_path_str).into_owned())
		.map(|path_str| PathBuf::from(path_str))
		.filter(|cfg_path| cfg_path.exists())
		.collect()
}

pub fn get_cfg_paths(shell: Shell) -> Vec<PathBuf> {
	match shell {
		Shell::BASH => check_paths_exist(vec![
			"~/.bash_profile",
			"~/.bash_login",
			"~/.profile",
			"~/.bashrc",
		]),
		Shell::ZSH => check_paths_exist(vec!["~/.zprofile", "~/.zlogin", "~/.zshrc", "~/.zshenv"]),
		Shell::FISH => check_paths_exist(vec!["~/.config/fish/config.fish"]),
		Shell::KSH => check_paths_exist(vec!["~/.profile", "~/.kshrc"]),
		Shell::CSH => check_paths_exist(vec!["~/.login", "~/.cshrc"]),
		Shell::TCSH => check_paths_exist(vec!["~/.login", "~/.tcshrc", "~/.cshrc"]),
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Var {
	GWD, // Working directory
	GSD, // States directory
	GDD, // Daemon directory
	GPD, // Project directory
}

impl Var {
	pub fn to_string(&self) -> &str {
		match *self {
			Var::GWD => "GWD",
			Var::GSD => "GSD",
			Var::GDD => "GDD",
			Var::GPD => "GPD",
		}
	}
	pub fn iterator() -> impl Iterator<Item = Var> {
		[Var::GWD, Var::GSD, Var::GDD, Var::GPD].iter().copied()
	}
}

pub fn replace_copy_env<E, LB, P>(env_var: E, replace_str: LB, file_path: P) -> Result<(), EnvError>
where
	E: Display,
	LB: AsRef<[u8]> + Display,
	P: AsRef<Path>,
{
	let file = File::open(&file_path)?;

	let reader = BufReader::new(&file);

	let mut env_found = false;

	let mut lines = reader
		.lines()
		.map(|line| {
			let line = line.expect("Line is unreadable.");
			if line.starts_with(&format!("{}=", env_var)) {
				env_found = true;
				replace_str.to_string()
			} else {
				line
			}
		})
		.collect::<Vec<_>>();

	if !env_found {
		lines.push("\n".to_string());
		lines.push(String::from("# Added by Godwit."));
		lines.push(replace_str.to_string());
	}

	let mut file = File::create(&file_path)?;

	for line in lines {
		file.write_all(line.as_bytes())?;

		file.write_all(b"\n")?;
	}

	Ok(())
}

pub fn set_env_var<EV>(var: Var, value: EV) -> Result<(), EnvError>
where
	EV: Display + Default,
{
	if cfg!(target_os = "windows") {
		Command::new("setx")
			.arg(var.to_string())
			.arg(value.to_string())
			.output()?;
	} else {
		let env_line = format!("{}=\"{}\"", var.to_string(), value);

		for shell in Shell::iterator() {
			for cfg_path in get_cfg_paths(shell) {
				replace_copy_env(var.to_string(), &env_line, cfg_path)?;
			}
		}
	}

	Ok(())
}

pub fn set_env_shell_var<EV>(shell: Shell, var: Var, value: EV) -> Result<(), EnvError>
where
	EV: Display + Default,
{
	if cfg!(target_os = "windows") {
		Command::new("setx")
			.arg(var.to_string())
			.arg(value.to_string())
			.output()?;
	} else {
		let env_line = format!("{}=\"{}\"", var.to_string(), value);

		for cfg_path in get_cfg_paths(shell) {
			replace_copy_env(var.to_string(), &env_line, cfg_path)?;
		}
	}

	Ok(())
}
