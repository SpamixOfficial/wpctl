use std::{env::args, process::exit};


mod app;
mod ui; // Note that this file needs to be imported for any ui functions to be available!!
        

fn main() {
    // Handle all of our possible arguments
    handle_args();

    // Creates the cursive root - required for every application.
    let mut app = app::App::new();
    app.init();
    app.ui_init();
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
