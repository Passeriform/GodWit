#![allow(unused_must_use)]
/// TUI Splash Core
///
/// TUI splash procedure core. Dictates the tui working and methods.

/// Structs/Enums:
///    DrawState -> A persistent struct used as global state manager for the tui draw cycles

/// Functions:
///    run -> Single call TUI bootstrapping.
///    draw -> On-demand redraw canvas with draw-state.
///    check_update -> Continuous poll for checking state updates.
///    up_pressed -> Detect up key-press.
///    down_pressed -> Detect down key-press.
///    enter_pressed -> Detect enter key-press.

use std::{
    io,
    time::Duration,
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{
        Text,
        Block,
        Widget,
        Borders,
        Paragraph,
        SelectableList,
    },
    layout::{
        Layout,
        Direction,
        Alignment,
        Constraint,
    },
    style::{
        Color,
        Style,
        Modifier,
    }
};
use crossterm::{
    terminal,
    event::{
        Event,
        KeyCode,
        read,
        poll
    },
};
use crate::iohandler::scanner;

pub struct DrawState<'a> {
    list_items: Vec<&'a str>,
    selected_item: usize,
}

pub fn run() -> Result<(), io::Error> {
    terminal::enable_raw_mode();
    let backend = CrosstermBackend::new(io::stdout());
    let mut term = Terminal::new(backend).unwrap();

    term.clear();

    let mut draw_state = DrawState {
        list_items: vec!["Item 1", "Item 2", "Item 3"],
        selected_item: 0,
    };

    draw(&mut term, &draw_state).unwrap();

    loop {
        check_update(&mut term, &mut draw_state);
    };
}

fn up_pressed(state: &mut DrawState) {
    if state.selected_item != 0 {
        state.selected_item -= 1;
    }
}

fn down_pressed(state: &mut DrawState) {
    if state.selected_item+1 != state.list_items.len() {
        state.selected_item += 1;
    }
}

fn enter_pressed(state: &mut DrawState) {
    state.selected_item += 2;
}

pub fn check_update(mut term: &mut Terminal<CrosstermBackend<io::Stdout>>, mut draw_state: &mut DrawState) -> Result<(), crossterm::ErrorKind> {
    if poll(Duration::from_millis(100))? {
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Up {
                    up_pressed(&mut draw_state);
                }
                if event.code == KeyCode::Down {
                    down_pressed(&mut draw_state);
                }
                if event.code == KeyCode::Enter {
                    enter_pressed(&mut draw_state);
                }
                draw(&mut term, &mut draw_state);
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, _) => println!("{:?}", width),
        }
    }
    Ok(())
}

pub fn draw(term: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &DrawState) -> Result<(), io::Error> {
    term.hide_cursor();

    term.draw(|mut f| {
        let size = f.size();

        // Wrapping Block
        Block::default()
        .borders(Borders::ALL)
        .render(&mut f, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ].as_ref()
            )
            .split(f.size());

        let art_para = scanner::get_art_para("assets/godwit_text_title_thin.ans", f.size().width as usize).unwrap();

        Paragraph::new(art_para.into_iter().map(|line| Text::raw(line)).collect::<Vec<Text>>().iter())
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
    terminal::disable_raw_mode();
    Ok(())
}
