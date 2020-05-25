//! TUI Splash Core
//!
//! TUI splash procedure core. Dictates the TUI windows and operations.
#![allow(unused_must_use)]
use crate::iohandler::scanner;
use crossterm::{
	event::{poll, read, Event, KeyCode},
	terminal,
};
use log::debug;
use std::{io, process, time::Duration};
use tui::{
	backend::CrosstermBackend,
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget},
	Terminal,
};

// TODO: Refactor

/// Global-state for TUI
pub struct DrawState<'a> {
	list_items: Vec<&'a str>,
	selected_item: usize,
}
/// TUI bootstrap
pub fn run() -> Result<(), io::Error> {
	terminal::enable_raw_mode();
	let backend = CrosstermBackend::new(io::stdout());
	let mut term = Terminal::new(backend).unwrap();

	term.clear();

	let mut draw_state = DrawState {
		list_items: vec!["Init", "Add", "Remove", "Switch"],
		selected_item: 0,
	};

	draw(&mut term, &draw_state).unwrap();

	loop {
		check_update(&mut term, &mut draw_state);
	}
}

/// Up key-press subroutine.
fn up_pressed(state: &mut DrawState) {
	if state.selected_item != 0 {
		state.selected_item -= 1;
	}
}

/// Down key-press subroutine.
fn down_pressed(state: &mut DrawState) {
	if state.selected_item + 1 != state.list_items.len() {
		state.selected_item += 1;
	}
}

/// Enter key-press subroutine.
fn enter_pressed(_state: &mut DrawState) {
	// Move to next window
}

/// Exit subroutine.
fn exit_routine(term: &mut Terminal<CrosstermBackend<io::Stdout>>, _state: &mut DrawState) {
	// Add exit coroutines and subwindows
	term.clear();
	term.show_cursor();
	terminal::disable_raw_mode();
	debug!("Nice chirping... Bye!");
	process::exit(1);
}

/// Poll for events and call draw on capture.
pub fn check_update(
	mut term: &mut Terminal<CrosstermBackend<io::Stdout>>,
	mut draw_state: &mut DrawState,
) -> Result<(), crossterm::ErrorKind> {
	if poll(Duration::from_millis(100))? {
		match read()? {
			Event::Key(event) => {
				match event.code {
					KeyCode::Up => up_pressed(&mut draw_state),
					KeyCode::Down => down_pressed(&mut draw_state),
					KeyCode::Enter => enter_pressed(&mut draw_state),
					KeyCode::Esc => exit_routine(&mut term, &mut draw_state),
					_ => (),
				}
				draw(&mut term, &mut draw_state);
			}
			Event::Mouse(event) => debug!("{:?}", event),
			Event::Resize(width, _) => {
				debug!("{:?}", width);
				draw(&mut term, &mut draw_state);
			}
		}
	}
	Ok(())
}

/// Redraw canvas using global-state.
pub fn draw(
	term: &mut Terminal<CrosstermBackend<io::Stdout>>,
	state: &DrawState,
) -> Result<(), io::Error> {
	term.hide_cursor();

	term.draw(|mut f| {
		let size = f.size();

		Block::default().borders(Borders::ALL).render(&mut f, size);

		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
			.split(f.size());

		let art_para =
			scanner::parse_art("assets/ansi/art.ans", f.size().width as usize).unwrap_or_default();

		Paragraph::new(
			art_para
				.into_iter()
				.map(|line| Text::raw(line))
				.collect::<Vec<Text>>()
				.iter(),
		)
		.style(Style::default().fg(Color::White).bg(Color::Black))
		.alignment(Alignment::Center)
		.wrap(true)
		.render(&mut f, chunks[0]);

		SelectableList::default()
			.block(Block::default().title("Operation").borders(Borders::ALL))
			.items(&state.list_items)
			.select(Some(state.selected_item))
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().modifier(Modifier::ITALIC))
			.highlight_symbol(">>")
			.render(&mut f, chunks[1]);
	});
	Ok(())
}