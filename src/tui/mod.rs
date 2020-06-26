//! TUI Splash Core
//!
//! TUI splash procedure core. Dictates the TUI windows and operations.
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
	widgets::{Block, Borders, List, Paragraph, Text, Widget},
	Terminal,
};

/// Global-state for TUI
pub struct DrawState<'a> {
	list_items: &'a [&'a str],
	selected_item: usize,
}
/// TUI bootstrap
pub fn run() -> Result<(), crossterm::ErrorKind> {
	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(io::stdout());
	let mut term = Terminal::new(backend).unwrap();

	term.clear()?;

	let mut draw_state = DrawState {
		list_items: &["Init", "Add", "Remove", "Switch"],
		selected_item: 0,
	};

	draw(&mut term, &draw_state).unwrap();

	loop {
		check_update(&mut term, &mut draw_state)?;
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
fn exit_routine(
	term: &mut Terminal<CrosstermBackend<io::Stdout>>,
	_state: &mut DrawState,
) -> Result<(), crossterm::ErrorKind> {
	// Add exit coroutines and subwindows
	term.clear()?;
	term.show_cursor()?;
	terminal::disable_raw_mode()?;
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
					KeyCode::Esc => exit_routine(&mut term, &mut draw_state)?,
					_ => (),
				}
				draw(&mut term, &mut draw_state)?;
			}
			Event::Mouse(event) => debug!("{:?}", event),
			Event::Resize(width, _) => {
				debug!("{:?}", width);
				draw(&mut term, &mut draw_state)?;
			}
		}
	}
	Ok(())
}

/// Redraw canvas using global-state.
pub fn draw(
	term: &mut Terminal<CrosstermBackend<io::Stdout>>,
	state: &DrawState,
) -> Result<(), crossterm::ErrorKind> {
	term.hide_cursor()?;

	term.draw(|mut f| {
		let size = f.size();

		let block = Block::default().borders(Borders::ALL);
		f.render_widget(block, size);

		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
			.split(f.size());

		let art_para = scanner::parse_art("assets/ansi/art.ans", f.size().width as usize)
			.unwrap_or_default()
			.into_iter()
			.map(|line| Text::raw(line))
			.collect::<Vec<Text>>();

		let art_widget = Paragraph::new(
			art_para.iter()
		)
		.style(Style::default().fg(Color::White).bg(Color::Black))
		.alignment(Alignment::Center)
		.wrap(true);

		f.render_widget(art_widget, chunks[0]);

		let operations = List::new(state.list_items.iter().map(|item| Text::raw(*item)))
			.block(Block::default().title("Operation").borders(Borders::ALL))
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().modifier(Modifier::ITALIC))
			.highlight_symbol(">>");

		f.render_widget(operations, chunks[1]);
	})?;
	Ok(())
}
