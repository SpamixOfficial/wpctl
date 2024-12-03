pub mod pages;
pub mod setup;

use std::io::{self, Error};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal, Frame,
};

use crate::{app::App, backend::repository::Remote};

pub fn run(mut app: App) -> anyhow::Result<()> {
    if !app.is_setup {
        setup::run(app.config_path.clone(), app.approot.clone())?;
        ratatui::restore();
        return Ok(());
    }
    let mut terminal: DefaultTerminal = ratatui::init();

    loop {
        pre_draw_handle(&mut app)?;

        terminal.draw(|frame| {
            // Get callback function and call function, looks cursed I know
            app.current_page.func()(frame, &mut app)
        })?;

        match post_draw_handle(&mut app) {
            Err(e) => {
                if e.to_string() == "brk" {
                    ratatui::restore();
                    return Ok(());
                }
                ratatui::restore();
                return Err(anyhow::Error::from(e));
            }
            _ => (),
        }
    }
}

fn pre_draw_handle(app: &mut App) -> anyhow::Result<()> {
    Ok(())
}

fn post_draw_handle(app: &mut App) -> anyhow::Result<()> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Err(anyhow::Error::from(Error::other("brk"))),
                KeyCode::Down => next_ui_list_item(app, false)?,
                KeyCode::Up => next_ui_list_item(app, true)?,
                _ => (),
            }
        }
    }

    return Ok(());
}

/// Direction true is up, false is down
fn next_ui_list_item(app: &mut App, direction: bool) -> anyhow::Result<()> {
    let chosen_index = app.ui_list_items.iter().position(|x| x.0 == true);
    let change = match direction {
        true => -1,
        false => 1,
    };
    if let Some(i) = chosen_index {
        if (i == app.ui_list_items.len() - 1 && !direction) || (i == 0 && direction) {
            return Ok(());
        }
        app.ui_list_items[i] = (false, app.ui_list_items[i].clone().1);
        app.ui_list_items[(i as isize + change) as usize] = (
            true,
            app.ui_list_items[(i as isize + change) as usize].clone().1,
        );
    } else {
        // This should never happen, but if it does we just set all to false and set the first item
        // to true
        app.ui_list_items = app
            .ui_list_items
            .iter_mut()
            .enumerate()
            .map(|(i, x)| {
                if i == 0 {
                    (true, x.1.clone())
                } else {
                    (false, x.1.clone())
                }
            })
            .collect();
    };
    Ok(())
}

#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Default,
}

impl Page {
    fn func(&self) -> fn(&mut Frame, &mut App) {
        // Add all pages here!!
        let func = match self {
            Self::Default => Self::page_main,
        };
        return func;
    }
}
