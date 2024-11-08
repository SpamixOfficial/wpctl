pub mod setup;
pub mod pages;

use std::io::{self, Error};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
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
        // Add all pages here!!
        let func = match self {
            Self::Default => Self::page_main,
        };
        return func;
    } 
}
