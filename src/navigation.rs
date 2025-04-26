use ratatui::text::Line;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, Default, Display, Clone, Copy, FromRepr, EnumIter, PartialEq, Eq)]
pub enum SelectedTab {
    #[strum(to_string = "Chat")]
    #[default]
    FirstTab,
    #[strum(to_string = "Inventory")]
    SecondTab,
    #[strum(to_string = "Blanks")]
    ThridTab,
}

impl SelectedTab {
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
