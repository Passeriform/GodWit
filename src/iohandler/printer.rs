//! Printer
//!
//! Controls input and read operations. Utility abstraction over general stdio input/read operations.
use crate::errors::{IOError, StateError};
use crate::statehandler::State;
use prettytable::{cell, format, row, Table};

/// Print pretty state-graphs.
pub fn print_state_graph(state_list: Vec<State>, verbose: bool) -> Result<(), IOError> {
	if state_list.is_empty() {
		return Err(StateError::EmptyStateList.into());
	}

	let mut table = Table::new();

	let format = format::FormatBuilder::new()
		.column_separator('│')
		.borders('│')
		.separator(
			format::LinePosition::Top,
			format::LineSeparator::new('─', '┬', '┌', '┐'),
		)
		.separator(
			format::LinePosition::Intern,
			format::LineSeparator::new('─', '┼', '├', '┤'),
		)
		.separator(
			format::LinePosition::Bottom,
			format::LineSeparator::new('─', '┴', '└', '┘'),
		)
		.padding(1, 1)
		.build();

	table.set_format(format);

	if verbose {
		table.set_titles(row![bic => "Project", "Location", "Status"]);

		for state in state_list {
			table.add_row(row![c =>
				state.get_glyph(),
				format!("{}", state.get_directory().unwrap_or_default().display()),
				format!("{:?}", state.get_status().unwrap_or_default()),
			]);
		}
	} else {
		table.set_titles(row![bic => "Project", "Location"]);

		for state in state_list {
			table.add_row(row![c =>
				state.get_glyph(),
				format!("{}", state.get_directory().unwrap_or_default().display()),
			]);
		}
	}

	table.printstd();
	Ok(())
}
