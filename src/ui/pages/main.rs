use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

use crate::ui::Page;

impl Page {
    pub fn page_main(frame: &mut Frame) {
        let [sidebar_area, main_area] =
            Layout::horizontal(Constraint::from_percentages([25, 75])).areas(frame.area());
        let default_block = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        // Render sidebar
        frame.render_widget(
            Paragraph::new("Yo this is da sidebar")
                .block(default_block.clone().title_top("Sidebar")),
            sidebar_area,
        );

        // Render main details area
        frame.render_widget(
            Paragraph::new("Yo this is da main area")
                .block(default_block.clone().title_top("Main area")),
            main_area,
        );
    }
}
