use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

use super::{Action, EventState};

pub struct HelpComponent {
    pub bindings: Vec<(&'static str, &'static str)>,
}

impl HelpComponent {
    pub fn new(bindings: Vec<(&'static str, &'static str)>) -> Self {
        Self { bindings }
    }

    /// Returns the (width, height) the popup needs, including borders and padding.
    pub fn dimensions(&self) -> (u16, u16) {
        let content_width = self
            .bindings
            .iter()
            .map(|(key, desc)| key.len().max(12) + desc.len())
            .max()
            .unwrap_or(0) as u16;
        // borders (2) + horizontal margin (2*2)
        let width = content_width + 6;
        // borders (2) + vertical margin (2*1)
        let height = self.bindings.len() as u16 + 4;
        (width, height)
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(Clear, area);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Keybindings ")
            .bold()
            .title_alignment(Alignment::Center);
        f.render_widget(block, area);

        let inner = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        let rows: Vec<Line> = self
            .bindings
            .iter()
            .map(|(key, desc)| {
                Line::from(vec![
                    Span::styled(format!("{:<12}", key), Style::new().yellow().bold()),
                    Span::raw(*desc),
                ])
            })
            .collect();

        f.render_widget(Paragraph::new(rows), inner);
    }

    pub fn handle_action(&mut self, _action: Action) -> EventState {
        EventState::NotConsumed
    }
}
