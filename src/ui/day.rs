use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::Day;

static ITEM_HEIGHT: u16 = 5;
static WEEK_HEIGHT: u16 = 3;

pub struct DayAnime {
    pub day: Day,
}

impl DayAnime {
    fn get_widget_height(&self) -> u16 {
        let list_len: u16 = self.day.anime.len() as u16;

        // outer border
        2 +
        // item border
        list_len +
        // item height
        ( list_len * ( ITEM_HEIGHT - 1 ) ) +
        // week title height
        WEEK_HEIGHT
    }
}

impl Widget for DayAnime {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut constraints = vec![Constraint::Length(WEEK_HEIGHT)];
        let mut day_constraints = self
            .day
            .anime
            .iter()
            .map(|_| Constraint::Length(ITEM_HEIGHT))
            .collect::<Vec<Constraint>>();
        constraints.append(&mut day_constraints);

        let outer = Layout::default()
            .constraints([
                Constraint::Max(self.get_widget_height()),
                Constraint::Min(0),
            ])
            .split(area);

        let margin = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(outer[0]);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(margin[1]);

        Paragraph::new(format!("\n{}", self.day.week)).render(layout[0], buf);

        self.day.anime.iter().enumerate().for_each(|(i, anime)| {
            if layout.get(i + 1).is_some() {
                Paragraph::new(format!("{}\n{}\n\n{}", anime.time, anime.name, anime.ep))
                    .block(Block::default().borders(Borders::TOP))
                    .render(layout[i + 1], buf);
            }
        });

        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.day.is_today {
                Color::Yellow
            } else {
                Color::White
            }))
            .render(outer[0], buf);
    }
}
