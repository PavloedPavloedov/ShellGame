use ratatui::{
    buffer::Buffer,
    layout::{Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};
use shellgame_style::style;

#[derive(Clone)]
struct Message {
    sender: String,
    message: String,
}

pub struct InputField {
    user: String,
    messages: Option<Vec<Message>>,
}

impl Message {
    pub fn new(sender: String, message: String) -> Self {
        Self { sender, message }
    }
    pub fn format_message(&self) -> Text {
        Text::from(vec![Line::from(Span::from(self.message.clone()))])
    }
}

impl InputField {
    pub fn new(user: String) -> Self {
        InputField {
            user,
            messages: None,
        }
    }
    pub fn message_list(&self) -> Option<Vec<ListItem>> {
        self.messages.as_ref().map(|messages| {
            messages
                .iter()
                .map(|message| ListItem::from(message.format_message()))
                .collect()
        })
    }
    pub fn new_message(&mut self, message: &Message) {
        self.messages
            .get_or_insert(vec![message.clone()])
            .push(message.clone());
    }
    fn render_list(&self, area: Rect, buf: &mut Buffer) {
        let message_list = self.message_list().unwrap_or(vec![]);

        StatefulWidget::render(
            List::new(message_list).block(
                Block::new()
                    .border_type(style::BORDERS_TYPE)
                    .borders(Borders::TOP),
            ),
            area,
            buf,
            &mut ListState::default(),
        );
    }
    fn render_paragraph(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(String::from(style::ENTER_MESSAGE))
            .block(
                Block::new()
                    .title(self.user.as_ref())
                    .borders(style::BORDERS)
                    .border_type(style::BORDERS_TYPE),
            )
            .render(area, buf);
    }
}

impl Widget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [list_area, paragraph_area] =
            Layout::vertical([style::TABS_CONSTRAINT, style::PAGES_CONSTRAINT]).areas(area);
        self.render_list(list_area, buf);
        self.render_paragraph(paragraph_area, buf);
    }
}
