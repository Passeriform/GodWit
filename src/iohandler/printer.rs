/// I/O Handler Printer
///
/// Printer module for I/O Handler.

/// Functions:
///   print_state_graph -> Prints state-graph status in a stylized format.

use crate::statehandler::StateGraph;

pub fn print_state_graph(state_list: StateGraph, verbose: Option<bool>) {
    println!("\nProjects currently tracked under Godwit:");
    match verbose {
        Some(true) => {
            println!("\n{:^20}|{:^20}|{:^20}", "Project", "Location", "Status");
            println!("{:=^60}","");

            for state in state_list.states() {
                println!("{:^20}|{:^20}|{:^20}", state.glyph, state.directory.unwrap().display(), state.status.unwrap());
            }
        },
        Some(false) | None => {
            println!("\n{:^20}|{:^20}", "Project", "Location");
            println!("{:=^40}","");

            for state in state_list.states() {
                println!("{:^20}|{:^20}", state.glyph, state.directory.unwrap().display());
            }
        }
    }
}
