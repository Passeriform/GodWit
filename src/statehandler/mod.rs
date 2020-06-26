//! Godwit State Handler
//!
//! A core state management utility for context switching and global singletons.
use crate::errors::StateError;
use crate::glyph::Glyph;
use crate::settings;
use getter_derive::Getter;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Status {
	// TODO: Elaborate enums into impl
	Active,
	Remote,
	Local,
	Tracking,
	Stale,
}

/// Defines a singular project and its corresponding state.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct State {
	pub glyph: Glyph,
	pub directory: Option<PathBuf>,
	pub status: Option<Vec<Status>>,
}

impl Default for State {
	fn default() -> Self {
		State {
			glyph: Default::default(),
			directory: Default::default(),
			status: Default::default(),
		}
	}
}

/// Defines combination of multiple states and represents the snapshot of the application.
#[derive(Debug, Deserialize, Serialize, Getter)]
#[serde(rename_all = "snake_case")]
pub struct StateGraph {
	default: Option<State>,
	active: Option<State>,
	states: Vec<State>,
	ignore: Vec<PathBuf>,
}

impl StateGraph {
	/// Returns new state-graph.
	pub fn init(
		default: Option<State>,
		active: Option<State>,
		states: Option<Vec<State>>,
		ignore: Option<Vec<PathBuf>>,
	) -> Self {
		StateGraph {
			default: default,
			active: active,
			states: states.unwrap_or_default(), // TODO: Templates
			ignore: ignore.unwrap_or_default(), // TODO: Templates
		}
	}

	/// Sets the active state in a state-graph.
	pub fn active(&mut self, state: &State) -> &mut Self {
		self.active = Some(state.clone());
		self
	}

	/// Sets the fallback state in a state-graph.
	pub fn fallback(&mut self, state: &State) -> &mut Self {
		self.default = Some(state.clone());
		self
	}

	/// Sets the working states in a state-graph.
	pub fn states(&mut self, states: &Vec<State>) -> &mut Self {
		self.states = states.to_vec();
		self
	}

	/// Appends a state in the working states list.
	pub fn append_state(&mut self, state: &State) -> &mut Self {
		self.states.push(state.clone());
		self
	}

	/// Drops state from the working states list.
	pub fn drop_state(&mut self, state: &State) -> &mut Self {
		self.states.retain(|p_state| p_state != state);
		self
	}
	/// Searches state by glyph and directory
	pub fn search_states(&self, q_term: String, fuzzy: bool) -> Option<State> {
		for state in &self.states {
			if fuzzy {
				if state
					.glyph
					.to_string()
					.to_lowercase()
					.contains(&q_term.to_lowercase())
					|| state
						.directory
						.clone()
						.unwrap_or_default()
						.to_string_lossy()
						.to_lowercase()
						.contains(&q_term.to_lowercase())
				{
					return Some(state.clone());
				}
			} else {
				if state.glyph.to_string().to_lowercase() == q_term.to_lowercase() {
					return Some(state.clone());
				}
			}
		}
		None
	}

	/// Commits changes to state-graph file.
	pub fn propagate(&self) -> Result<(), Box<dyn Error>> {
		File::create(settings::get_settings()?.get_save_state()?).and_then(|state_file| {
			serde_json::to_writer_pretty(state_file, &self)?;
			Ok(())
		})?;
		Ok(())
	}
}

impl Default for StateGraph {
	fn default() -> Self {
		StateGraph {
			default: Default::default(),
			active: Default::default(),
			states: Default::default(),
			ignore: Default::default(),
		}
	}
}

/// Returns a state-graph instance from the state-graph file.
pub fn load_stategraph() -> Result<StateGraph, Box<dyn Error>> {
	let state_graph =
		File::open(settings::get_settings()?.get_save_state()?).and_then(|state_file| {
			let state_graph: StateGraph = serde_json::from_reader(state_file)?;
			Ok(state_graph)
		})?;
	Ok(state_graph)
}

/// Sets the active state in state-graph and propagates it.
pub fn set_active(q_glyph: &Glyph) -> Result<(), Box<dyn Error>> {
	let mut sg_snapshot: StateGraph = load_stategraph()?;

	sg_snapshot
		.search_states(q_glyph.to_string(), true)
		.map_or_else(
			|| {
				Err(StateError::StateNotFound {
					state: q_glyph.clone().into(),
				}
				.into())
			},
			|q_state| {
				sg_snapshot.active(&q_state).propagate()?;
				Ok(())
			},
		)
}

/// Sets the default state in state-graph and propagates it.
pub fn set_default(q_glyph: &Glyph) -> Result<(), Box<dyn Error>> {
	let mut sg_snapshot: StateGraph = load_stategraph()?;

	sg_snapshot
		.search_states(q_glyph.to_string(), true)
		.map_or_else(
			|| {
				Err(StateError::StateNotFound {
					state: q_glyph.clone().into(),
				}
				.into())
			},
			|q_state| {
				sg_snapshot.fallback(&q_state).propagate()?;
				Ok(())
			},
		)
}

/// Adds a new state in state-graph.
pub fn add_state(
	glyph: Glyph,
	location: PathBuf,
	status: Option<Vec<Status>>,
	as_active: bool,
	as_default: bool,
) -> Result<(), Box<dyn Error>> {
	let mut sg_snapshot = load_stategraph().unwrap_or_default();

	if sg_snapshot
		.search_states(glyph.to_string(), false)
		.is_none()
		&& sg_snapshot
			.search_states(location.to_string_lossy().to_string(), false)
			.is_none()
	{
		let new_state = State {
			glyph: glyph,
			directory: Some(location),
			status: status,
		};

		let mut sg_snapshot = sg_snapshot.append_state(&new_state);

		if sg_snapshot.get_default().is_none() || as_default {
			sg_snapshot = sg_snapshot.fallback(&new_state);
		}

		if settings::get_settings()?.get_switch_on_add() || as_active {
			sg_snapshot = sg_snapshot.active(&new_state);
		}

		sg_snapshot.propagate()?;

		Ok(())
	} else {
		Err(StateError::StateAlreadyExists {
			state: glyph.into(),
		}
		.into())
	}
}

/// Removes the state from state-graph
pub fn purge_state(q_glyph: Glyph) -> Result<(), Box<dyn Error>> {
	let mut sg_snapshot: StateGraph = load_stategraph()?;

	sg_snapshot
		.search_states(q_glyph.to_string(), true)
		.map_or_else(
			|| {
				Err(StateError::StateNotFound {
					state: q_glyph.into(),
				}
				.into())
			},
			|q_state| {
				let mut sg_snapshot = sg_snapshot.drop_state(&q_state);

				if q_state == sg_snapshot.get_default().unwrap_or_default() {
					sg_snapshot = sg_snapshot.fallback(&sg_snapshot.get_states()[0]);
				}

				if q_state == sg_snapshot.get_active().unwrap_or_default() {
					sg_snapshot =
						sg_snapshot.active(&sg_snapshot.get_default().unwrap_or_default());
				}

				sg_snapshot.propagate()?;

				Ok(())
			},
		)
}
