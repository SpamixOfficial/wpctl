use std::{env::args, process::exit};

mod ui;

fn main() {
    // Handle all of our possible arguments
    handle_args();

    // Creates the cursive root - required for every application.
    let mut app = ui::App::new();
    app.init();
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
