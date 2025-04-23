use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};
use strum_macros::FromRepr;

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq)]
pub enum SelectedTab {
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

impl SelectedTab {
    // pub fn render_title(self) -> Line<'static> {
        
    // }

    pub fn previous(self) -> Self {
        Self::from_repr((self as usize).saturating_sub(1)).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        Self::from_repr((self as usize).saturating_add(1)).unwrap_or(self)
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // match self {
        //     Self::FirstTab => self.render_chat(area, buf),
        //     Self::SecondTab => self.render_inventory(area, buf),
        //     Self::ThridTab => self.render_blanks(area, buf),
        //     Self::Fourthtab => self.render_tab_1(area, buf),
        // }
    }
}
