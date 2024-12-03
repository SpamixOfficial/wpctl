use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};
use ratatui_image::StatefulImage;
use reqwest::StatusCode;

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
        let current_item = packages.iter().find(|x| x.0).unwrap().1.to_owned();

        let sidebar = List::new(list_items);

        // Render sidebar
        frame.render_widget(
            sidebar.block(
                default_block
                    .clone()
                    .title_top("Packages")
                    .title_top(Line::from("<Enter to open menu>").right_aligned())
                    .title_bottom(Line::from("<Arrows to navigate>").right_aligned()),
            ),
            sidebar_area,
        );

        // Render main details area
        let [img_area, details_area] = Layout::default()
            .constraints(Constraint::from_percentages([50, 50]))
            .direction(ratatui::layout::Direction::Vertical)
            .margin(2)
            .spacing(2)
            .areas(main_area);
        let details = current_item.as_lines_ui();

        // TODO, reqwest is too blocking
        // load up image from thumbnail_url
        /*let image_bytes = match reqwest::blocking::get(current_item.thumbnail_url.clone()) {
            Ok(x) => {
                if x.status() == StatusCode::OK {
                    Some(x.bytes().unwrap())
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        let image = match image_bytes {
            Some(x) => Some(image::load_from_memory(&x).unwrap().to_rgb8()),
            None => None,
        };*/

        /*frame.render_stateful_widget(
            StatefulImage::new(None),
            img_area,
            &mut app.ui_current_thumbnail,
        );*/
        frame.render_widget(
            Paragraph::new(details).wrap(ratatui::widgets::Wrap { trim: true }),
            details_area,
        );
        frame.render_widget(default_block.clone().title_top("Details"), main_area);
    }
}
