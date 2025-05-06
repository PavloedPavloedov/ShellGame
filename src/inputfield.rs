use ratatui::{layout::Alignment, text::Line, widgets::BorderType};

struct InputField<'a> {
    title: Line<'a>,
    title_alg: Alignment,
    message: Vec<Line<'a>>,
    msg_title: Line<'a>,
    msg_alg: Alignment,
    border_type: BorderType,
}

impl<'a> InputField<'a> {
    pub fn new(
        new_title: Line<'a>,
        new_title_alg: Alignment,
        new_msg_title: Line<'a>,
        new_msg_alg: Alignment,
        new_border_type: BorderType,
    ) -> Self {
        InputField {
            title: new_title,
            title_alg: new_title_alg,
            message: vec![Line::from("")],
            msg_title: new_msg_title,
            msg_alg: new_msg_alg,
            border_type: new_border_type,
        }
    pub fn add_line(&mut self, line: Line<'a>) {
        self.message.push(line);
    }
}
//     pub fn change_msg(&mut self, msg: &str) {
//         self.message.push_str(msg);
//     }
//     pub fn change_style(&mut self, new_style: Style) {
//         self.msg_style = new_style;
//     }
//     pub fn change_alignment(&mut self, new_alignment: Alignment) {
//         self.alignment = new_alignment;
//     }
//     pub fn get_msg_len(&self) -> usize {
//         self.message.len()
//     }
//     pub fn get_style(&self) -> Style {
//         self.msg_style.clone()
//     }
//     pub fn get_alignment(&self) -> Alignment {
//         self.alignment.clone()
//     }
// }

impl Widget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::styled(self.message, self.msg_style))
            .block(
                Block::new()
                    .title(self.title)
                    .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
                    .border_type(BORDERS_TYPE),
            )
            .render(area, buf);
    }
}
