use std::{
    default,
    io::{self, Error, ErrorKind},
    process::exit,
};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::app::App;

pub fn run(mut app: App) -> io::Result<()> {
    let mut terminal: DefaultTerminal = ratatui::init();
    if !app.is_setup {
        app.current_page = Page::SetupWelcome
    }

    loop {
        pre_draw_handle(&mut app)?;

        terminal.draw(|frame| {
            // Get callback function and call function, looks cursed I know
            app.current_page.func()(frame)
        })?;

        match post_draw_handle(&mut app) {
            Err(e) => {
                if e.to_string() == "brk" {
                    return Ok(());
                }
                return Err(e);
            }
            _ => (),
        }
    }
}

fn pre_draw_handle(app: &mut App) -> io::Result<()> {
    Ok(())
}

fn post_draw_handle(app: &mut App) -> io::Result<()> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Err(Error::other("brk")),
                KeyCode::Right => {
                    if !app.is_setup {
                        if app.current_page == Page::SetupPage1 {
                            App::install(app.config.clone(), app.approot.clone())?;
                        }
                        if let Ok(pg) = app.current_page.next() {
                            app.current_page = pg
                        };
                    }
                },
                KeyCode::Left => {
                    if !app.is_setup {
                        if let Ok(pg) = app.current_page.back() {
                            app.current_page = pg
                        };
                    }
                }
                _ => (),
            }
        }
    }

    return Ok(());
}


#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Default,
    SetupWelcome,
    SetupPage1,
    SetupPage2
}

impl Page {
    fn func(&self) -> fn(&mut Frame) {
        let func = match self {
            Self::SetupWelcome => Self::page_setup_welcome,
            Self::SetupPage1 => Self::page_setup_1,
            Self::SetupPage2 => Self::page_setup_2,
            _ => Self::page_default,
        };
        return func;
    }

    fn next(&self) -> Result<Self, ()> {
        match self {
            Self::SetupWelcome => Ok(Self::SetupPage1),
            Self::SetupPage1 => Ok(Self::SetupPage2),
            Self::SetupPage2 => Ok(Self::Default),
            _ => Err(()),
        }
    }

    fn back(&self) -> Result<Self, ()> {
        match self {
            Self::SetupPage1 => Ok(Self::SetupWelcome),
            _ => Err(()),
        }
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
        [area] = Layout::vertical([Constraint::Length(15 as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::new()
            .title_top(Line::from("Setup wpctl").centered())
            .title_bottom(Line::from("Left to continue").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(text).centered().block(block), area);
    }

    fn page_setup_1(frame: &mut Frame) {
        let bullet_point_style = Style::default().bold().underline_color(Color::Yellow);
        let text = Text::from(vec![
            Line::from("The following actions will take place:"),
            Line::from("* App directories will be created").style(bullet_point_style),
            Line::from("* App configurations will be created").style(bullet_point_style),
        ]);
        let [mut area] = Layout::horizontal([Constraint::Length((text.width() as u16) + 5)])
            .flex(Flex::Center)
            .areas(frame.area());
        [area] = Layout::vertical([Constraint::Length(15 as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::new()
            .title_top(Line::from("Setup wpctl").centered())
            .title_bottom(Line::from("Left to continue").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(text).block(block), area);
    }
    
    fn page_setup_2(frame: &mut Frame) {
        let bullet_point_style = Style::default().bold().underline_color(Color::Yellow);
        let text = Text::from(vec![
            Line::from("The actions have been done!"),
            Line::from("Press right to finish the setup").style(bullet_point_style),
        ]);
        let [mut area] = Layout::horizontal([Constraint::Length((text.width() as u16) + 5)])
            .flex(Flex::Center)
            .areas(frame.area());
        [area] = Layout::vertical([Constraint::Length(15 as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::new()
            .title_top(Line::from("Setup wpctl").centered())
            .title_bottom(Line::from("Left to continue").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(text).block(block), area);
    }
}
