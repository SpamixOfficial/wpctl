use std::{env::args, process::exit};


mod app;
mod ui;
// Old, early cursive TUI thingys, moving to ratatui now :-)
//mod old/ui; // Note that this file needs to be imported for any ui functions to be available!!
        

fn main() {
    // Handle all of our possible arguments
    handle_args();

    // Creates the cursive root - required for every application.
    let mut app = app::App::new();
    app.init();
    match ui::run(app) {
        Err(e) => {
            eprintln!("Uh oh! Something went wrong with the ui. Exiting...\n{}", e.to_string());
            exit(1)
        },
        _ => ()
    }
    // old stuff again 
    //app.ui_init();
}

fn handle_args() {
    let args: Vec<String> = args().collect::<Vec<String>>()[1..].to_vec();
    for arg in args {
        match arg.to_lowercase().as_str() {
            "--help" | "-h" => help(),
            _ => (),
        }
    }
}

fn help() {
    println!(
        r"Usage: wpctl [OPTIONS]
wpctl is a commandline tool made for handling your LDW installation

Options:
    -h | --help     Help!!!

SpamixOfficial 2024"
    );
    exit(0);
}
