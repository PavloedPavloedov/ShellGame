struct InputField {
    message: Option<String>,
    character_sender: Option<String>,
}

static TITLE: String = "Введите сообщение";
static BORDERS: ratatui::widgets::Borders = ratatui::widgets::Borders::ALL;
static BORDERS_TYPE: ratatui::widgets::BorderType = ratatui::widgets::BorderType::Double;

impl InputField {
    fn new(character_name: String) -> Self {
        InputMessage {}
    }
    fn enter_message(self, message: &str) -> Self {
        self.message
            .get_or_insert(String::new())
            .push_str(message.to_string());
        self
    }
}

impl Winget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ratatui::Block::new()
            .title(TITLE)
            .border(BORDERS)
            .border_type(BORDERS_TYPE)
    }
}
