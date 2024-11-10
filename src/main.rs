use app::App;
use clap::{Parser, Subcommand};
use std::process::exit;

mod app;
mod ui;
mod backend;
mod utils;
// Old, early cursive TUI thingys, moving to ratatui now :-)
//mod old/ui; // Note that this file needs to be imported for any ui functions to be available!!

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "Package manager for wallpapers", long_about = None, arg_required_else_help(false))]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(long_about = "Run the installer in cli")]
    Install,
}

fn main() {
    // Handle all of our possible arguments
    //handle_args();
    let args = Args::parse();

    // Creates the root - required for every application.
    let mut app = App::new();
    app.init();

    // We actually parse our commands here before the ui is init
    handle_cmd(args, &app);

    match ui::run(app) {
        Err(e) => {
            eprintln!(
                "Uh oh! Something went wrong with the ui. Exiting...\n{}",
                e.to_string()
            );
            exit(1)
        }
        _ => (),
    }
    // old stuff again
    //app.ui_init();
}

fn handle_cmd(args: Args, app: &App) {
    if let Some(command) = args.command {
        let func: fn(&App) = match command {
            Command::Install => command_install,
        };
        func(app);
    }
}

fn command_install(app: &App) {
    match App::install(app.config_path.clone(), app.approot.clone()) {
        Err(e) => {
            eprintln!("[*] Error during installation: {}", e);
            exit(1);
        }
        _ => {
            println!("[*] Installation is done!");
            exit(0);
        }
    };
}
