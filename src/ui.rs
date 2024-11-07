use std::{default, io::{self, Error, ErrorKind}, process::exit};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::app::App;

#[derive(Debug, Default)]
pub enum Page {
    #[default]
    Default,
    SetupWelcome,
    SetupPage1,
}

pub fn run(mut app: App) -> io::Result<()> {
    let mut terminal: DefaultTerminal = ratatui::init();
    loop {
        pre_draw_handle(&mut app)?;

        terminal.draw(|frame| {
            // Get callback function and call function, looks cursed I know
            app.current_page.func()(frame)
        })?;
    
        match post_draw_handle(&mut app) {
            Err(e) => {
                if e.to_string() == "brk" {
                    return Ok(())
                }
                return Err(e)
            },
            _ => ()
        }
    }
}

fn pre_draw_handle(app: &mut App) -> io::Result<()> {
    // Check if app is set up
    if !app.is_setup {
        app.current_page = Page::SetupWelcome
    }

    Ok(())
}

fn post_draw_handle(app: &mut App) -> io::Result<()> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            return Err(Error::other("brk"));
        }
    }
    return Ok(())
}

impl Page {
    fn func(&self) -> fn(&mut Frame) {
        let func = match self {
            Self::SetupWelcome => Self::page_setup_welcome,
            _ => Self::page_default,
        };
        return func;
    }

    fn page_default(frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new("Wpctl is started...").centered(),
            frame.area(),
        );
    }

    fn page_setup_welcome(frame: &mut Frame) {
        let text = Text::from(
        "Hey!\n\n\n\nLooks like you haven't went through the setup yet!\nPress left to continue",
    );
        let [mut area] = Layout::horizontal([Constraint::Length((text.width() as u16) + 5)])
            .flex(Flex::Center)
            .areas(frame.area());
        [area] = Layout::vertical([Constraint::Length(20 as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::new()
            .title_top(Line::from("Setup wpctl").centered())
            .title_bottom(Line::from("Left to continue").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(text).centered().block(block), area);
    }
}
