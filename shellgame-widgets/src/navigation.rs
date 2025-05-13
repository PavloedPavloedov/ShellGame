use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Tabs, Widget},
};
use shellgame_style::style;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, Default, Display, Clone, Copy, FromRepr, EnumIter, PartialEq, Eq)]
pub enum TabState {
    #[strum(to_string = "Chat")]
    #[default]
    Chat,
    #[strum(to_string = "Inventory")]
    Inventory,
    #[strum(to_string = "Blanks")]
    Blanks,
}

impl TabState {
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").into()
    }
    pub fn previous(self) -> Self {
        match self as usize {
            0 => Self::Blanks,
            current => Self::from_repr((self as usize).saturating_sub(1)).unwrap(),
        }
    }
    pub fn next(self) -> Self {
        Self::from_repr((self as usize).saturating_add(1)).unwrap_or(Self::Chat)
    }
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        Tabs::new(
            TabState::iter()
                .map(|state| state.title())
                .collect::<Vec<Line>>(),
        )
        .divider(style::DIVIDER)
        .block(
            Block::default()
                .borders(style::BORDERS)
                .border_type(style::BORDERS_TYPE),
        )
        .select(self.clone() as usize)
        .render(area, buf);
    }
}

impl Widget for TabState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_tabs(area, buf);
    }
}
