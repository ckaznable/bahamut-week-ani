use std::io;

use crossterm::{event::{KeyCode, self, Event, DisableMouseCapture}, terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, execute};
use ratatui::{backend::{Backend, CrosstermBackend}, Frame, Terminal};

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

    f.render_widget(DayAnime {
        day: days.get(0).unwrap().clone(),
        is_today: false,
    }, size);
}
