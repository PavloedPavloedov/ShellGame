use ratatui::{
    layout::Constraint,
    widgets::{BorderType, Borders},
};

pub static DIVIDER: &str = " ";
pub static BORDERS: Borders = Borders::ALL;
pub static BORDERS_TYPE: BorderType = BorderType::Double;
pub static PAGES_CONSTRAINT: Constraint = Constraint::Percentage(100);
pub static TABS_CONSTRAINT: Constraint = Constraint::Min(3);
pub static ENTER_MESSAGE: &str = "Enter message";
pub static FIRST_RATIO: Constraint = Constraint::Ratio(1, 3);
pub static SECOND_RATIO: Constraint = Constraint::Ratio(2, 3);
