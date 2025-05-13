use ratatui::{
    buffer::Buffer,
    layout::{Layout, Rect}
    ,
    widgets::Widget,
};
use shellgame_style::style;
use shellgame_widgets::input_field::InputField;


pub struct Page {
    title: String,
}

impl Page {
    pub fn new(title: String) -> Self {
        Page { title }
    }
}

impl Widget for Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [first_area, second_area] = Layout::horizontal([style::FIRST_RATIO, style::SECOND_RATIO]).areas(area);

        InputField::new("Павел".to_string()).render(first_area, buf);
    }
}
