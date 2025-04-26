use ratatui::{
    buffer::Buffer,
    layout::{self, Layout, Rect},
    widgets::{Block, Widget},
};

static FIRST_PAGE_DIVISION: layout::Constraint = layout::Constraint::Ratio(1, 3);
static SECOND_PAGE_DIVISION: layout::Constraint = layout::Constraint::Ratio(2, 3);
static DIRECTION: layout::Direction = layout::Direction::Horizontal;
static BORDERS: ratatui::widgets::Borders = ratatui::widgets::Borders::ALL;
static BORDERS_TYPE: ratatui::widgets::BorderType = ratatui::widgets::BorderType::Double;

pub struct ThirdPage {
    name: String,
}

impl ThirdPage {
    pub fn new() -> Self {
        ThirdPage {
            name: "Пошёл нахуй".to_string(),
        }
    }
}

impl Widget for ThirdPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [first_area, second_area] = Layout::default()
            .direction(DIRECTION)
            .constraints([FIRST_PAGE_DIVISION, SECOND_PAGE_DIVISION])
            .areas(area);
        let [fisrt_block, second_block] = [
            Block::default().borders(BORDERS).border_type(BORDERS_TYPE),
            Block::default().borders(BORDERS).border_type(BORDERS_TYPE),
        ];
        fisrt_block.render(first_area, buf);
        second_block.render(second_area, buf);
    }
}
