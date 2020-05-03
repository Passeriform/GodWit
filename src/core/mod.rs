/// Godwit Core
///
/// Will be used as an API and general low level abstraction. Mostly with call forward wrappers.
/// It contains full API definitions and endpoints to integrate Godwit core.
///
/// Functions:
///    init -> One time godwit working setup call with settings return
///    add -> add project to godwit tracker
///    list -> list tracked projects
///    killsave -> remove previous active saved state-graph
///    switch -> switch the active project
///    runsplash -> trigger/forward the splash tui

mod setup;
pub mod splash;

use std::{
    io,
    path::PathBuf,
};
use crate::glyph::Glyph;
use crate::statehandler::{
    self,
    State,
    StateGraph
};
use crate::plugins;
use self::setup::{setup_gw_dir, setup_init_state};

pub fn init(path: Option<PathBuf>, headless: Option<bool>) -> Result<(), &'static str> {
    match setup_gw_dir(path, headless) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    match setup_init_state() {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    return Ok(());
}

pub fn add(glyph: Glyph, location: PathBuf, existing: Option<bool>) -> Result<(), &'static str> {
    match existing {
        Some(false) => {
            match plugins::invoke("Weaver", None) {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }
        Some(true) | None => ()
    }

    match statehandler::add_state(glyph, location) {
        Ok(_) => println!("State added"),
        Err(e) => return Err(e)
    };

    Ok(())
}

pub fn list() -> Result<StateGraph, &'static str> {
    return match statehandler::get_snapshot() {
        Ok(current_state) => Ok(current_state),
        Err(e) => Err(e)
    };
}

pub fn switch(glyph: Glyph) -> Result<(), &'static str> {
    let new_state: State = State {  // unimplemented!
        glyph: glyph.to_str(),
        ..Default::default()
    };

    statehandler::set_active(new_state);
    Ok(())
}

pub fn killsave() { // unimplemented!

}

pub fn runsplash() -> Result<(), io::Error> {
    return match splash::run() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    };
}
