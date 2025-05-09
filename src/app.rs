use crate::navigation::TabState;
use crate::secondpage::SecondPage;
use crate::thirdpage::ThirdPage;

use ratatui::{
    buffer::Buffer,
    crossterm::{event, event::KeyCode, event::KeyEvent},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal,
};
use std::{io, sync::mpsc};
use strum_macros::Display;

static PAGES_CONSTRAINT: Constraint = Constraint::Percentage(100);
static TABS_CONSTRAINT: Constraint = Constraint::Min(3);

#[derive(Default, Clone, Copy)]
pub struct App {
    state: AppState,
    page: TabState,
}

#[derive(Default, Display, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Exit,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal, rx: mpsc::Receiver<KeyEvent>) -> io::Result<()> {
        while self.state == AppState::Running {
            if let Ok(key) = rx.recv() {
                match key.code {
                    event::KeyCode::Char('q') | event::KeyCode::Char('й') | KeyCode::Esc => self.quit(),
                    event::KeyCode::Right | event::KeyCode::Char('d') => self.next_tab(),
                    event::KeyCode::Left | event::KeyCode::Char('a') => self.previous_tab(),
                    _ => ()
                }
            }
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }
        Ok(())
    }
    fn quit(&mut self) {
        self.state = AppState::Exit;
    }
    fn next_tab(&mut self) {
        self.page = self.page.next();
    }
    fn previous_tab(&mut self) {
        self.page = self.page.previous();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tab_title_area, pages_area] =
            Layout::vertical([TABS_CONSTRAINT, PAGES_CONSTRAINT]).areas(area);

        self.page.render(tab_title_area, buf);

        match self.page {
            TabState::FirstTab => (),
            TabState::SecondTab => SecondPage::new().render(pages_area, buf),
            TabState::ThridTab => ThirdPage::new().render(pages_area, buf),
        }
    }
}
