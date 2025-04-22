use ratatui::{
    DefaultTerminal,
    layout::Constraint::{self}
};
use strum_macros::{FromRepr, Display};
use std::io;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct App {
    #[default]
    Running,
    Exit,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(self) -> bool {
        self.state == AppState::Running
    }

    fn quit(&mut self) {
        self.state = AppState::Exit;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                event::KeyCode::Char('q') | event::KeyCode::Char('й') | KeyCode::Esc => {
                    self.quit()
                }
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }
}