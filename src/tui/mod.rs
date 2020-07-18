//! TUI Splash Core
//!
//! TUI splash procedure core. Dictates the TUI windows and operations.
pub mod event;
pub mod gwidget;

use crate::errors::TuiError;
use crate::iohandler::scanner;
use crate::tui::event::{TuiEvent, TuiEvents};
use crate::tui::gwidget::StatefulList;
use crossterm::cursor::{
	DisableBlinking, EnableBlinking, Hide, RestorePosition, SavePosition, Show,
};
use crossterm::event::{Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use log::debug;
use std::io::{self, Write};
use std::process;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text};
use tui::Terminal;

/// State managing app struct
struct App<'a> {
	operations: StatefulList<&'a str>,
}

impl<'a> App<'a> {
	fn new() -> App<'a> {
		App {
			operations: StatefulList::with_items(vec!["Init", "Add", "Remove", "Switch"]), //pull this dynamically
		}
	}
	fn advance(&mut self) {}
}

/// TUI bootstrap
pub fn run() -> Result<(), TuiError> {
	execute!(
		io::stdout(),
		SavePosition,
		DisableBlinking,
		Hide,
		EnterAlternateScreen
	)?;

	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(io::stdout());
	let mut terminal = Terminal::new(backend)?;

	let events = TuiEvents::new();
	let mut app = App::new();

	app.operations.next();

	loop {
		terminal.draw(|mut f| {
			let block = Block::default().borders(Borders::ALL);
			f.render_widget(block, f.size());

			let columns = Layout::default()
				.direction(Direction::Horizontal)
				.margin(1)
				.constraints([Constraint::Ratio(4, 12), Constraint::Ratio(8, 12)].as_ref())
				.split(f.size());

			let sidebar = Layout::default()
				.direction(Direction::Vertical)
				.margin(1)
				.constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
				.split(columns[0]);

			let style = Style::default().fg(Color::White).bg(Color::Black);

			let art_para = scanner::parse_art("assets/art/icon.asc", f.size().width as usize)
				.unwrap_or_default()
				.into_iter()
				.map(|line| Text::raw(line))
				.collect::<Vec<Text>>();

			let art_widget = Paragraph::new(art_para.iter())
				.style(Style::default().fg(Color::White).bg(Color::Black))
				.alignment(Alignment::Center)
				.wrap(true);

			f.render_widget(art_widget, sidebar[0]);

			let operations = List::new(app.operations.items.iter().map(|item| Text::raw(*item)))
				.block(Block::default().title("Operation").borders(Borders::ALL))
				.style(style)
				.highlight_style(
					Style::default()
						.fg(Color::LightGreen)
						.modifier(Modifier::ITALIC),
				)
				.highlight_symbol(" ➤ ");

			f.render_stateful_widget(operations, sidebar[1], &mut app.operations.state);
		})?;

		match events.next()? {
			TuiEvent::Input(event) => match event {
				Event::Key(key) => match key.code {
					KeyCode::Up => app.operations.previous(),
					KeyCode::Down => app.operations.next(),
					KeyCode::Enter => push_routine(app.operations.state.selected()),
					KeyCode::Esc => exit_routine()?,
					_ => (),
				},
				Event::Mouse(mouse) => debug!("{:?}", mouse),
				_ => (),
			},
			TuiEvent::Tick => {
				app.advance();
			}
			_ => (),
		}
	}
}

/// Progress to next window
fn push_routine<VT>(_stdin: VT) {
	()
}

/// Exit subroutine.
fn exit_routine() -> Result<(), crossterm::ErrorKind> {
	// Add exit coroutines and subwindows
	execute!(
		io::stdout(),
		LeaveAlternateScreen,
		Show,
		EnableBlinking,
		RestorePosition,
	)?;
	terminal::disable_raw_mode()?;

	debug!("Nice chirping... Bye!");
	process::exit(1);
}
