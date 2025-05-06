use crate::inputfield::InputField;
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Direction, Layout, Rect},
    symbols::{border, line},
    widgets::{Block, BorderType, Borders, Widget},
};

static BORDERS_TYPE: BorderType = BorderType::Double;
static VERTIC_DIRECTION: Direction = Direction::Vertical;
static HORIZON_DIRECTION: Direction = Direction::Horizontal;
static FIRST_RATIO: Constraint = Constraint::Ratio(1, 3);
static SECOND_RATIO: Constraint = Constraint::Ratio(2, 3);
static BORDER_SET: border::Set = border::Set {
    top_right: line::DOUBLE_HORIZONTAL_DOWN,
    bottom_right: line::DOUBLE_HORIZONTAL_UP,
    ..border::DOUBLE
};

pub struct ChatPage {
    title: Option<String>,
    inv_widgt: Widget,
}

impl Page {
    pub fn new() -> Self {
        Page { title: None }
    }
}

impl Widget for Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [first_area, second_area] = Layout::default()
            .direction(HORIZON_DIRECTION)
            .constraints([FIRST_RATIO, SECOND_RATIO])
            .areas(area);

        let [chat_msg_area, chat_msg_input_area] = Layout::default()
            .direction(VERTIC_DIRECTION)
            .constraints([
                ratatui::prelude::Constraint::Percentage(20
                ),
                ratatui::prelude::Constraint::Percentage(80
                ),
            ])
            .areas(second_area);

        let [fisrt_block, second_block] = [
            Block::default()
                .borders(Borders::ALL)
                .border_type(BORDERS_TYPE)
                .border_set(BORDER_SET),
            Block::default()
                .borders(Borders::TOP | Borders::RIGHT)
                .border_type(BORDERS_TYPE)
                .border_set(BORDER_SET),
        ];

        fisrt_block.render(first_area, buf);
        second_block.render(chat_msg_area, buf);
    }
}
