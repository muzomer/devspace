use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{
        palette::tailwind::{AMBER, CYAN},
        Style, Stylize,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

use super::{Action, EventState};

pub enum HelpEntry {
    Binding(&'static str, &'static str),
    Section(&'static str),
    Blank,
}

pub struct HelpComponent {
    pub entries: Vec<HelpEntry>,
}

impl HelpComponent {
    pub fn new(entries: Vec<HelpEntry>) -> Self {
        Self { entries }
    }

    /// Returns the (width, height) the popup needs, including borders and padding.
    pub fn dimensions(&self) -> (u16, u16) {
        let content_width = self
            .entries
            .iter()
            .map(|e| match e {
                HelpEntry::Binding(key, desc) => key.len().max(12) + desc.len(),
                HelpEntry::Section(title) => title.len(),
                HelpEntry::Blank => 0,
            })
            .max()
            .unwrap_or(0) as u16;
        // borders (2) + horizontal margin (2*2)
        let width = content_width + 6;
        // borders (2) + vertical margin (2*1) + one extra blank line per Section
        let section_count = self
            .entries
            .iter()
            .filter(|e| matches!(e, HelpEntry::Section(_)))
            .count() as u16;
        let height = self.entries.len() as u16 + section_count + 4;
        (width, height)
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(Clear, area);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(super::POPUP_BORDER_STYLE)
            .style(super::POPUP_BG_STYLE)
            .title(Line::from(" Help ").style(Style::new().fg(CYAN.c300).bold()))
            .title_alignment(Alignment::Center);
        f.render_widget(block, area);

        let inner = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        let rows: Vec<Line> = self
            .entries
            .iter()
            .flat_map(|e| match e {
                HelpEntry::Binding(key, desc) => vec![Line::from(vec![
                    Span::styled(format!("{:<12}", key), Style::new().fg(AMBER.c300).bold()),
                    Span::raw(*desc),
                ])],
                HelpEntry::Section(title) => vec![
                    Line::from(Span::styled(
                        *title,
                        Style::new().fg(CYAN.c400).bold().underlined(),
                    )),
                    Line::raw(""),
                ],
                HelpEntry::Blank => vec![Line::raw("")],
            })
            .collect();

        f.render_widget(Paragraph::new(rows), inner);
    }

    pub fn handle_action(&mut self, _action: Action) -> EventState {
        EventState::NotConsumed
    }
}
