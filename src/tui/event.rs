use crossterm::event::{read, Event, KeyCode};
use log::error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

pub enum TuiEvent {
	Input(Event),
	Resize(u16, u16),
	Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
#[allow(dead_code)]
pub struct TuiEvents {
	rx: mpsc::Receiver<TuiEvent>,
	input_handle: thread::JoinHandle<()>,
	ignore_exit_key: Arc<AtomicBool>,
	tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
	pub exit_key: KeyCode,
	pub tick_rate: Duration,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			exit_key: KeyCode::Esc,
			tick_rate: Duration::from_millis(250),
		}
	}
}

impl TuiEvents {
	pub fn new() -> TuiEvents {
		TuiEvents::with_config(Config::default())
	}

	pub fn with_config(config: Config) -> TuiEvents {
		let (tx, rx) = mpsc::channel();
		let ignore_exit_key = Arc::new(AtomicBool::new(false));
		let input_handle = {
			let tx = tx.clone();
			let ignore_exit_key = ignore_exit_key.clone();
			thread::spawn(move || loop {
				match read().unwrap() {
					Event::Key(event) => {
						if let Err(err) = tx.send(TuiEvent::Input(Event::Key(event))) {
							error!("{}", err); // Returning error types is inconsequential when utilizing rx
							return;
						}
						if !ignore_exit_key.load(Ordering::Relaxed) && event.code == config.exit_key
						{
							return;
						}
					}
					Event::Mouse(event) => {
						if let Err(err) = tx.send(TuiEvent::Input(Event::Mouse(event))) {
							error!("{}", err); // Returning error types is inconsequential when utilizing rx
							return;
						}
					}
					_ => (),
				}
			})
		};
		let tick_handle = {
			thread::spawn(move || loop {
				tx.send(TuiEvent::Tick).unwrap();
				thread::sleep(config.tick_rate);
			})
		};
		TuiEvents {
			rx,
			ignore_exit_key,
			input_handle,
			tick_handle,
		}
	}

	pub fn next(&self) -> Result<TuiEvent, mpsc::RecvError> {
		self.rx.recv()
	}

	pub fn disable_exit_key(&mut self) {
		self.ignore_exit_key.store(true, Ordering::Relaxed);
	}

	pub fn enable_exit_key(&mut self) {
		self.ignore_exit_key.store(false, Ordering::Relaxed);
	}
}
