mod cli;
use std::io;
mod devspace;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

fn main() {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    print!("{}", app_result.unwrap());
}

#[derive(Debug)]
pub struct App {
    devspaces: devspace::DevspaceList,
    exit: bool,
    selected_space: String,
}

impl Default for App {
    fn default() -> Self {
        let args = cli::Args::new();
        let items = devspace::list(&args.spaces_dir).unwrap_or_default();

        Self {
            devspaces: devspace::DevspaceList::new(items),
            exit: false,
            selected_space: String::new(),
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<String> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(self.selected_space)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Enter => self.go_to_devspace(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.devspaces.state.select_next();
    }
    fn select_previous(&mut self) {
        self.devspaces.state.select_previous();
    }

    fn select_first(&mut self) {
        self.devspaces.state.select_first();
    }

    fn select_last(&mut self) {
        self.devspaces.state.select_last();
    }

    fn go_to_devspace(&mut self) {
        if let Some(selected_index) = self.devspaces.state.selected() {
            let selected_space = &self.devspaces.items[selected_index];
            self.selected_space = selected_space.clone();
        }
        self.exit();
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Devspacs")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_devspaces(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().borders(Borders::all());
        let list = List::new(self.devspaces.items.clone())
            .block(block)
            .style(Style::new().white())
            .highlight_style(SELECTED_STYLE)
            .direction(ListDirection::TopToBottom);
        StatefulWidget::render(list, area, buf, &mut self.devspaces.state);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ or j/k to move, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_devspaces(main_area, buf);
    }
}
