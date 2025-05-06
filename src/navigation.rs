use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Tabs, Widget},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

static DIVIDER: &str = " ";
static BORDERS: ratatui::widgets::Borders = ratatui::widgets::Borders::ALL;
static BORDERS_TYPE: ratatui::widgets::BorderType = ratatui::widgets::BorderType::Double;

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
    pub fn draw(self, selectedtab: TabState, area: Rect, buf: &mut Buffer) {
        self.tab().select(selectedtab as usize).render(area, buf);
    }
    fn tab(self) -> Tabs<'static> {
        Tabs::new(TabState::iter().map(TabState::title))
            .divider(DIVIDER)
            .block(self.block(BORDERS, BORDERS_TYPE))
    }
    fn block(self, border: Borders, bordertype: BorderType) -> Block<'static> {
        Block::default().borders(border).border_type(bordertype)
    }
}
