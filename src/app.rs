mod navigation;

use crate::app::navigation::SelectedTab;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::Widget,
};
use std::io;

#[derive(Default)]
pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, PartialEq, Eq)]
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
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.selected_tab.render(area, buf);
    }
}
