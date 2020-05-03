/// Godwit State Handler
///
/// A core state management utility for context switching and global singletons.
///
/// Structs/Enums:
///    State -> Defines a singular project and its corresponding state.
///    StateGraph -> Defines combination of multiple states and represents the snapshot of the application.
///
/// Implementation methods:
///    new -> Returns new state-graph struct.
///    active -> Sets the active state in a state-graph.
///    fallback -> Sets the fallback state in a state-graph.
///    states -> Returns a Vec<States> listing tracked states.
///    append_state -> Appends a given state in the tracked states list.
///    propagate -> Reflects the changes in a state-graph to the state-graph source file.
///
/// Functions:
///    get_snapshot -> Returns a current state-graph from the state-graph source file.
///    set_active -> Sets the active state in state-graph and propagates it.
///    set_default -> Sets the default state in state-graph and propagates it.
///    get_state_list -> Returns a Vec<States> listing tracked states.
///    add_state -> Adds a new state to the state-graph and propagates it.

use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::settings::fetch;
use crate::glyph::Glyph;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct StateGraph {
    default: State,
    active: State,
    states: Vec<State>,
    ignore: Vec<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub glyph: String,
    pub directory: Option<PathBuf>,
    pub status: Option<String>,
}

impl Default for State {
  fn default() -> Self {
    State {
        glyph: "@default/default".to_string(),
        directory: None,
        status: None,
    }
  }
}

impl StateGraph {
    pub fn active(self, state: State) -> Self {
        StateGraph {
            active: state,
            ..self
        }
    }
    pub fn fallback(self, state: State) -> Self {
        StateGraph {
            default: state,
            ..self
        }
    }
    pub fn states(self) -> Vec<State> {
        return self.states;
    }
    pub fn append_state(self, state: State) -> Self {
        let mut state_list = self.states;
        state_list.push(state);

        StateGraph {
            states: state_list,
            ..self
        }
    }
    pub fn propagate(self) -> Result<(), &'static str> {
        let json_file = File::create(fetch::get_settings()?.get_save_state()).expect("File wasn't found");

        println!("{:?}", fetch::get_settings()?.get_save_state());
        serde_json::to_writer_pretty(json_file, &self).expect("Writing file threw error");
        Ok(())
    }
}

impl Default for StateGraph {
  fn default() -> Self {
    StateGraph {
        default: State::default(),
        active: State::default(),
        states: vec![State::default()],
        ignore: vec![PathBuf::default()]
    }
  }
}

pub fn get_snapshot() -> Result<StateGraph, &'static str> {
    let json_file = File::open(fetch::get_settings()?.get_save_state()).expect("File wasn't found");
    let state_graph: StateGraph = serde_json::from_reader(json_file).expect("State JSON is invalid");  //Solution is a bit fucky
    return Ok(state_graph);
}

pub fn set_active(state_item: State) {
    let sg_snapshot: StateGraph = get_snapshot().unwrap_or(StateGraph::default());

    sg_snapshot.active(state_item).propagate().expect("Error occured in state propagation.");
}

pub fn set_default(state_item: State) {
    let sg_snapshot: StateGraph = get_snapshot().unwrap_or(StateGraph::default());

    sg_snapshot.fallback(state_item).propagate().expect("Error occured in state propagation.");
}

pub fn get_state_list(snapshot: StateGraph) -> Vec<State> {
    return snapshot.states;
}

pub fn add_state(glyph: Glyph, location: PathBuf) -> Result<(), &'static str> {
    let sg_snapshot = get_snapshot().unwrap_or(StateGraph::default());

    let new_state = State {glyph: glyph.to_str(), directory: Some(location), ..Default::default()};

    sg_snapshot.append_state(new_state).propagate().expect("Error occured in appending states.");
    Ok(())
}
