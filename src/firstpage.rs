use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Widget},
};

static BORDERS: Borders = Borders::ALL;
static BORDERS_TYPE: BorderType = BorderType::Double;

pub struct FirstPage {
    name: String,
}

impl FirstPage {
    pub fn new() -> Self {
        FirstPage {
            name: "Пошёл нахуй".to_string(),
        }
    }
}

impl Widget for FirstPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [first_area, second_area] = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
            .areas(area);
        let [fisrt_block, second_block] = [
            Block::default().borders(BORDERS).border_type(BORDERS_TYPE),
            Block::default().borders(BORDERS).border_type(BORDERS_TYPE),
        ];
        fisrt_block.render(first_area, buf);
        second_block.render(second_area, buf);
    }
}
