mod firstpage;
mod navigation;
mod secondpage;
mod thirdpage;

use crate::firstpage::FirstPage;
use crate::navigation::SelectedTab;
use crate::secondpage::SecondPage;
use crate::thirdpage::ThirdPage;

use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{self, Layout, Rect},
    style::palette::tailwind,
    widgets::{Block, Tabs, Widget},
};
use std::io;
use strum::IntoEnumIterator;

static DIVIDER: &str = " ";
static LEFT_PADDING: &str = " ";
static RIGHT_PADDING: &str = " ";
static BASE_DIRECTION: layout::Direction = layout::Direction::Vertical;
static BORDERS: ratatui::widgets::Borders = ratatui::widgets::Borders::ALL;
static PAGES_CONSTRAINT: layout::Constraint = layout::Constraint::Percentage(100);
static TABS_CONSTRAINT: layout::Constraint = layout::Constraint::Min(3);
static BORDERS_TYPE: ratatui::widgets::BorderType = ratatui::widgets::BorderType::Double;
static BG: ratatui::prelude::Color = tailwind::GRAY.c900;

fn main() -> io::Result<()> {
    let app_result = App::default().run(ratatui::init());
    ratatui::restore();
    app_result
}

#[derive(Default, Clone, Copy)]
pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
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
            self.main_handle_events()?;
        }
        Ok(())
    }

    fn main_handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.press_handle_events(key),
            _ => {}
        }
        Ok(())
    }

    fn press_handle_events(&mut self, key: event::KeyEvent) {
        match key.code {
            event::KeyCode::Char('q') | event::KeyCode::Char('й') | KeyCode::Esc => self.quit(),
            event::KeyCode::Right | event::KeyCode::Char('d') => self.next_tab(),
            event::KeyCode::Left | event::KeyCode::Char('a') => self.previous_tab(),
            _ => (),
        }
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
        let [tab_title_area, pages_area] = Layout::default()
            .direction(BASE_DIRECTION)
            .constraints([TABS_CONSTRAINT, PAGES_CONSTRAINT])
            .areas(area);

        let tabs_title = SelectedTab::iter().map(SelectedTab::title);
        let border = Block::default().borders(BORDERS).border_type(BORDERS_TYPE);
        Tabs::new(tabs_title)
            .highlight_style((ratatui::style::Color::default(), BG))
            .select(self.selected_tab as usize)
            .divider(DIVIDER)
            .block(border)
            .render(tab_title_area, buf);

        match self.selected_tab {
            SelectedTab::FirstTab => FirstPage::new().render(pages_area, buf),
            SelectedTab::SecondTab => SecondPage::new().render(pages_area, buf),
            SelectedTab::ThridTab => ThirdPage::new().render(pages_area, buf),
        }
    }
}
