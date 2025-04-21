use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{self},
        Direction, Flex, Layout, Rect,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
};

pub mod tab_ui {
    mod navigation;
    use navigation::navigation_logic::tab_logic::TabManager;
    impl Widget for TabManager {
        fn create_widget_from_tab_list() {
        }
        fn render_tab_list() {
        }
    }
}