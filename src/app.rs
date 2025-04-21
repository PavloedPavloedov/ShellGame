#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct App {
    #[default]
    Running,
    Exit,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
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

fn main() -> io::Result<()> {
    let app_result = App::default().run(ratatui::init());
    ratatui::restore();
    app_result
}