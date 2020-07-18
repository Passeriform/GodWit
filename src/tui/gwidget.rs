use tui::widgets::ListState;

pub struct StatefulList<IT> {
	pub state: ListState,
	pub items: Vec<IT>,
}

impl<IT> StatefulList<IT> {
	pub fn new() -> StatefulList<IT> {
		StatefulList {
			state: ListState::default(),
			items: Vec::new(),
		}
	}

	pub fn with_items(items: Vec<IT>) -> StatefulList<IT> {
		StatefulList {
			state: ListState::default(),
			items,
		}
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len() - 1 {
					0
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len() - 1
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
	}

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}
