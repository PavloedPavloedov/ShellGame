use crate::navigation_logic::SelectedTab;

use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{self},
        Direction, Flex, Layout, Rect,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
};

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::FirstTab => self.render_chat(area, buf),
            Self::SecondTab => self.render_inventory(area, buf),
            Self::ThridTab => self.render_blanks(area, buf),
            Self::Fourthtab => self.render_tab_1(area, buf),
        }
    }
}

impl SelectedTab {
    fn render_chat(self, area: Rect, buf: &mut Buffer) {
    }
    fn render_inventory(self, area: Rect, buf: &mut Buffer) {
    }
    fn render_blanks(self, area: Rect, buf: &mut Buffer) {
    }
    fn render_tab_1(self, area: Rect, buf: &mut Buffer) {
    }
}