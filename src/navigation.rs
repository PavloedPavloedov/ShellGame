use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Tabs, Widget},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

static DIVIDER: &str = " ";
static BORDERS: Borders = Borders::ALL;
static BORDERS_TYPE: BorderType = BorderType::Double;

#[derive(Debug, Default, Display, Clone, Copy, FromRepr, EnumIter, PartialEq, Eq)]
pub enum TabState {
    #[strum(to_string = "Chat")]
    #[default]
    FirstTab,
    #[strum(to_string = "Inventory")]
    SecondTab,
    #[strum(to_string = "Blanks")]
    ThridTab,
}

impl TabState {
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").into()
    }
    pub fn previous(self) -> Self {
        Self::from_repr((self as usize).saturating_sub(1)).unwrap_or(Self::ThridTab)
    }
    pub fn next(self) -> Self {
        Self::from_repr((self as usize).saturating_add(1)).unwrap_or(Self::FirstTab)
    }
}

impl Widget for TabState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Tabs::new(TabState::iter().map(TabState::title))
            .divider(DIVIDER)
            .block(Block::default()
            .borders(BORDERS)
            .border_type(BORDERS_TYPE))
            .select(self as usize)
            .render(area, buf);
    }
}