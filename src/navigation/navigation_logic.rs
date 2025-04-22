use strum_macros::{FromRepr, Display};

//Структура менеждера вкладок, который содержит в себе текущую активную вкладку,
//и список высех вкладок
#[derive(Default, Clone, Copy, Display, FromRepr)]
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
    //Получение предыдущей вкладки, если такой не существует, возращается текущая
    fn previous(self) -> Self {
        Self::from_repr((self as usize).saturating_sub(1)).unwrap_or(self)
    }
    //Получение следующей вкладки, если такой не существует, возращается текущая
    fn next(self) -> Self {
        Self::from_repr((self as usize).saturating_add(1)).unwrap_or(self)
    }
}
