use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};

use crate::{app::App, ui::Page};

impl Page {
    pub fn page_main(frame: &mut Frame, app: &mut App) {
        let [sidebar_area, main_area] =
            Layout::horizontal(Constraint::from_percentages([25, 75])).areas(frame.area());
        let default_block = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let packages = app.ui_list_items.clone();
        let list_items = packages.iter().map(|x| {
            let mut item = ListItem::new(x.1.name.clone());
            if x.0 {
                item = item.bg(Color::Gray).fg(Color::Black)
            } else {
                item = item.bg(Color::default()).fg(Color::default())
            };
            item
        });
        let sidebar = List::new(list_items);

        // Render sidebar
        frame.render_widget(
            sidebar
                .block(default_block.clone().title_top("Packages")),
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
