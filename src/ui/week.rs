use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::Day;

use super::day::DayAnime;

static WEEK_ITEM_WIDTH: u16 = 20;

pub struct WeekAnime {
    day: Vec<Day>,
}

impl WeekAnime {
    pub fn new(day: Vec<Day>) -> WeekAnime {
        WeekAnime { day }
    }

    fn get_widget_width(&self) -> u16 {
        let len = self.day.len() as u16;
        // week item width
        WEEK_ITEM_WIDTH * len
    }
}

impl Widget for WeekAnime {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let widget_width = self.get_widget_width();
        let padding = if area.width > widget_width {
            (area.width - self.get_widget_width()) / 2
        } else {
            0
        };

        let mut constraints = vec![Constraint::Length(padding)];
        let mut items_constraints: Vec<Constraint> = self
            .day
            .iter()
            .map(|_| Constraint::Length(WEEK_ITEM_WIDTH))
            .collect();
        constraints.append(&mut items_constraints);
        constraints.push(Constraint::Min(0));
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        self.day.iter().enumerate().for_each(|(i, day)| {
            if layout.get(i + 1).is_some() {
                DayAnime::new(day.clone()).render(layout[i + 1], buf);
            }
        });
    }
}
