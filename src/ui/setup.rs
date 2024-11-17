use std::{
    default, io::{self, Error, ErrorKind}, path::PathBuf, process::exit
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

struct SetupSession {
    config: PathBuf,
    approot: PathBuf,
    current_page: SetupPage,
}

pub fn run(config: PathBuf, approot: PathBuf) -> anyhow::Result<()> {
    let mut terminal: DefaultTerminal = ratatui::init();
    let mut setup_session = SetupSession {
        config,
        approot,
        current_page: SetupPage::SetupWelcome,
    };

    loop {
        terminal.draw(|frame| {
            // Get callback function and call function, looks cursed I know
            setup_session.current_page.func()(frame)
        })?;

        match post_draw_handle(&mut setup_session) {
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

fn post_draw_handle(app: &mut SetupSession) -> anyhow::Result<()> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Err(anyhow::Error::from(Error::other("brk"))),
                KeyCode::Right => {
                    if app.current_page == SetupPage::SetupPage1 {
                        App::install(app.config.clone(), app.approot.clone())?;
                    }
                    if let Ok(pg) = app.current_page.next() {
                        app.current_page = pg
                    };
                }
                KeyCode::Left => {
                    if let Ok(pg) = app.current_page.back() {
                        app.current_page = pg
                    };
                }
                _ => (),
            }
        }
    }

    return Ok(());
}

#[derive(Debug, PartialEq)]
enum SetupPage {
    SetupWelcome,
    SetupPage1,
    SetupPage2,
}

impl SetupPage {
    fn func(&self) -> fn(&mut Frame) {
        let func = match self {
            Self::SetupWelcome => Self::page_setup_welcome,
            Self::SetupPage1 => Self::page_setup_1,
            Self::SetupPage2 => Self::page_setup_2,
        };
        return func;
    }

    fn next(&self) -> Result<Self, ()> {
        match self {
            Self::SetupWelcome => Ok(Self::SetupPage1),
            Self::SetupPage1 => Ok(Self::SetupPage2),
            Self::SetupPage2 => {exit(0);},
        }
    }

    fn back(&self) -> Result<Self, ()> {
        match self {
            Self::SetupPage1 => Ok(Self::SetupWelcome),
            _ => Err(()),
        }
    } 

    fn page_setup_welcome(frame: &mut Frame) {
        let text = Text::from(
        "Hey!\n\n\n\nLooks like you haven't went through the setup yet!\nPress right to continue",
    );
        let [mut area] = Layout::horizontal([Constraint::Length((text.width() as u16) + 5)])
            .flex(Flex::Center)
            .areas(frame.area());
        [area] = Layout::vertical([Constraint::Length(15 as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::new()
            .title_top(Line::from("Setup wctl").centered())
            .title_bottom(Line::from("Right to continue").centered())
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
            .title_top(Line::from("Setup wctl").centered())
            .title_bottom(Line::from("Right to continue").centered())
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
            .title_top(Line::from("Setup wctl").centered())
            .title_bottom(Line::from("Right to continue").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(text).block(block), area);
    }
}
