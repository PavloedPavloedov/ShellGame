use std::io;



impl Widget for App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let [base_up_area, base_bottom_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Percentage(100)])
            .flex(Flex::Start)
            .areas(area);
        let [add_up_area, add_bottom_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
            .areas(base_bottom_area);
        
        Tabs::new(Tab::to_tabs())
            .block(
                Block::new()
                    .title("Tabs")
                    .border_type(BorderType::Double)
                    .borders(Borders::ALL),
            )
            .divider("|")
            .render(base_up_area, buf);

        Paragraph::new("Top")
            .block(Block::new().border_type(BorderType::Double).borders(Borders::ALL))
            .render(add_up_area, buf);
        Paragraph::new("Top")
            .block(Block::new().border_type(BorderType::Double).borders(Borders::ALL))
            .render(add_bottom_area, buf);
    }
}

