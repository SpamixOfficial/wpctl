pub mod setup;

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
    if !app.is_setup {
        setup::run(app.config.clone(), app.approot.clone())?;
        ratatui::restore();
        return Ok(())
    }
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
                    ratatui::restore();
                    return Ok(());
                }
                ratatui::restore();
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
                _ => (),
            }
        }
    }

    return Ok(());
}


#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Default
}

impl Page {
    fn func(&self) -> fn(&mut Frame) {
        let func = match self {
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
}
