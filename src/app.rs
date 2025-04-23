mod navigation;

use crate::navigation::SelectedPage;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{self, Constraint, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders, Tabs, Widget},
};
use std::io;
use strum::IntoEnumIterator;

fn main() -> io::Result<()> {
    let app_result = App::default().run(ratatui::init());
    ratatui::restore();
    app_result
}

#[derive(Default, Clone, Copy)]
pub struct App {
    state: AppState,
    selected_tab: SelectedPage,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Exit,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                event::KeyCode::Char('q') | event::KeyCode::Char('й') | KeyCode::Esc => {
                    self.quit()
                }
                event::KeyCode::Right | event::KeyCode::Char('d') => self.next_tab(),
                event::KeyCode::Left | event::KeyCode::Char('a') => self.previous_tab(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Exit;
    }

    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn render_tab_title(self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedPage::iter().map(SelectedPage::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tab_title_area, pages_area] = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Percentage(100)])
            .areas(area);
        self.render_tab_title(tab_title_area, buf);
    }
}
