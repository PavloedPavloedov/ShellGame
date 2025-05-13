use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::{event, event::KeyCode, event::KeyEvent},
    layout::{Layout, Rect},
    widgets::Widget,
};
use std::{io, sync::mpsc};
use strum_macros::Display;

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
    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        rx: mpsc::Receiver<KeyEvent>,
    ) -> io::Result<()> {
        while self.state == AppState::Running {
            if let Ok(key) = rx.recv() {
                match key.code {
                    event::KeyCode::Char('q') | event::KeyCode::Char('й') | KeyCode::Esc => {
                        self.quit()
                    }
                    event::KeyCode::Right | event::KeyCode::Char('d') => self.next_tab(),
                    event::KeyCode::Left | event::KeyCode::Char('a') => self.previous_tab(),
                    _ => (),
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
            Layout::vertical([style::TABS_CONSTRAINT, style::PAGES_CONSTRAINT]).areas(area);

        self.page.render(tab_title_area, buf);

        match self.page {
            TabState::Chat => {
                pages::firstpage::Page::new("Nbgf".to_string()).render(pages_area, buf)
            }
            TabState::Inventory => pages::secondpage::SecondPage::new().render(pages_area, buf),
            TabState::Blanks => pages::thirdpage::ThirdPage::new().render(pages_area, buf),
        }
    }
}
