use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Widget},
};
use strum_macros::{Display, EnumIter, FromRepr};

static FIRST_RATIO: Constraint = Constraint::Ratio(1, 3);
static SECOND_RATIO: Constraint = Constraint::Ratio(2, 3);
static BORDERS: ratatui::widgets::Borders = ratatui::widgets::Borders::ALL;
static BORDERS_TYPE: ratatui::widgets::BorderType = ratatui::widgets::BorderType::Double;

#[derive(Debug, Default, Display, Clone, Copy, FromRepr, EnumIter, PartialEq, Eq)]
enum PageState {
    #[default]
    View, 
    Input, 
    ListItem,
    ListMsg
}

impl PageState {
    fn block(self, border: Borders, bordertype: BorderType) -> Block <'static> {
        Block::default().borders(border).border_type(bordertype)
    }
}

impl Widget for PageState {
    fn render(self, page_area: Rect, buf: &mut Buffer) {
        let [first_area, second_area] =
            Layout::horizontal([FIRST_RATIO, SECOND_RATIO]).areas(page_area);
        let [] = self.block().border_set(border_set), self.block(border, bordertype);
    }
}
