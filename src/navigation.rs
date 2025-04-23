use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize, palette::tailwind},
    text::Line,
    widgets::Widget,
};
use strum_macros::{Display, FromRepr, EnumIter};

#[derive(Debug, Default, Display, Clone, Copy, FromRepr, EnumIter, PartialEq, Eq)]
pub enum SelectedPage {
    #[strum(to_string = "Chat")]
    #[default]
    FirstTab,
    #[strum(to_string = "Inventory")]
    SecondTab,
    #[strum(to_string = "Blanks")]
    ThridTab,
    #[strum(to_string = "Tab 1")]
    Fourthtab,
}

impl SelectedPage {
    pub fn title(self) -> Line<'static> {
        format!("   {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub fn previous(self) -> Self {
        Self::from_repr((self as usize).saturating_sub(1)).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        Self::from_repr((self as usize).saturating_add(1)).unwrap_or(self)
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::FirstTab => tailwind::BLUE,
            Self::SecondTab => tailwind::EMERALD,
            Self::ThridTab => tailwind::INDIGO,
            Self::Fourthtab => tailwind::RED,
        }
    }
}

impl Widget for SelectedPage {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
