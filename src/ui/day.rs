use ratatui::{widgets::{Widget, Block, Borders, Paragraph}, layout::{Layout, Direction, Rect, Constraint}, buffer::Buffer, style::{Style, Color}};

use crate::Day;

pub struct DayAnime {
    pub day: Day,
    pub is_today: bool,
}

impl Widget for DayAnime {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut constraints = self.day.anime.iter()
            .map(|_| Constraint::Length(4))
            .collect::<Vec<Constraint>>();
        constraints.insert(0, Constraint::Length(3));

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        Paragraph::new(format!("\n{}", self.day.week))
            .render(layout[0], buf);

        self.day.anime
            .iter()
            .enumerate()
            .for_each(|(i, anime)| {
                if layout.get(i + 1).is_some() {
                    Paragraph::new(format!("{}\n{}\n\n{}", anime.time, anime.name, anime.ep))
                        .block(Block::default().borders(Borders::ALL))
                        .render(layout[i + 1], buf);
                }
            });
    }
}