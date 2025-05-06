use crate::navigation::TabState;
use crate::secondpage::SecondPage;
use crate::thirdpage::ThirdPage;

use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{self, Layout, Rect},
    widgets::Widget,
};
use std::io;
use std::{sync::Arc, sync::Mutex, sync::mpsc, thread};
use strum_macros::Display;

static PAGES_CONSTRAINT: layout::Constraint = layout::Constraint::Percentage(100);
static TABS_CONSTRAINT: layout::Constraint = layout::Constraint::Min(3);

fn 

#[derive(Default, Clone, Copy)]
pub struct App {
    state: AppState,
    tabstate: TabState,
}

#[derive(Default, Display, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Exit,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let (event_tx, event_rx) = mpsc::channel::<io::Result<()>>();
        let mutex_self = Arc::new(Mutex::new(self));

        let handle_self = Arc::clone(&mutex_self);
        let handle_tread = thread::spawn(move || {
            while (*handle_self.lock().unwrap()).state == AppState::Running {
                let _ = event_tx.send(self.main_handle_events());
            }
        });

        while (*mutex_self.lock().unwrap()).state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }

        handle_tread.join().unwrap();
        event_rx.recv().unwrap()
    }

    fn main_handle_events(tx: mpsc::Sender<Event>) {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
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
        self.tabstate = self.tabstate.next();
    }

    fn previous_tab(&mut self) {
        self.tabstate = self.tabstate.previous();
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        self.tabstate.draw(self.tabstate, area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tab_title_area, pages_area] =
            Layout::vertical([TABS_CONSTRAINT, PAGES_CONSTRAINT]).areas(area);

        self.render_tabs(tab_title_area, buf);

        match self.tabstate {
            TabState::FirstTab => (),
            TabState::SecondTab => SecondPage::new().render(pages_area, buf),
            TabState::ThridTab => ThirdPage::new().render(pages_area, buf),
        }
    }
}
