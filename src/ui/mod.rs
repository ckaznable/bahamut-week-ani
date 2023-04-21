use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crate::Day;

use self::day::DayAnime;

pub mod day;

pub fn run(days: &mut [Day]) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = draw(&mut terminal, days);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, days: &mut [Day]) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, days))?;

        if let Event::Key(event) = event::read()? {
            if let KeyCode::Char('q') = event.code {
                return Ok(());
            }
        };
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, days: &mut [Day]) {
    let size = f.size();

    let mut constraints: Vec<Constraint> = days.iter().map(|_| Constraint::Length(20)).collect();
    constraints.push(Constraint::Min(0));

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(size);

    days.iter().enumerate().for_each(|(i, day)| {
        f.render_widget(DayAnime { day: day.clone() }, layout[i]);
    });
}
