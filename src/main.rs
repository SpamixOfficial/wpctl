use app::App;
use clap::{Parser, Subcommand};
use clap_complete::{ArgValueCompleter, CompletionCandidate};
use std::process::exit;

mod app;
mod backend;
mod ui;
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
    #[command(long_about = "Add a repository from an URL pointing to a repository manifest")]
    AddRepo { url: String },
    #[command(long_about = "Remove a local repository from an identifier")]
    RemoveRepo {
        #[arg(add = ArgValueCompleter::new(identifier_clap_completer), help = "Package to remove, by identifier")]
        identifier: String,
    },
    #[command(
        long_about = "Get repository identifiers. Use the repositories command for repository information"
    )]
    Identifiers,
}

fn main() {
    // Handle all of our possible arguments
    //handle_args();
    let args = Args::parse();

    // Creates the root - required for every application.
    let mut app = App::new();
    app.init();

    // We actually parse our commands here before the ui is init
    handle_cmd(args, &mut app);

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

fn handle_cmd(args: Args, app: &mut App) {
    if let Some(command) = args.command {
        match command {
            Command::Install => command_install(app),
            Command::AddRepo { url } => add_repo(app, url),
            Command::RemoveRepo { identifier } => remove_repo(app, identifier),
            Command::Identifiers => identifiers(app),
        };
    }
}

fn command_install(app: &mut App) {
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

fn add_repo(app: &mut App, url: String) {
    match app.add_repo(url) {
        Err(e) => {
            eprintln!("[*] Error during repository addition: {}", e);
            exit(1);
        }
        _ => {
            println!("[*] Repository was added successfully!");
            exit(0);
        }
    }
}

fn identifiers(app: &mut App) {
    match App::identifiers(&app.config_path) {
        Err(e) => {
            eprintln!("[*] Error during identifers retrieval: {}", e);
            exit(1);
        }
        Ok(x) => {
            for i in x {
                println!("{i}");
            }
            exit(0);
        }
    };
}

fn remove_repo(app: &mut App, identifier: String) {
    match app.remove_repo_id(identifier) {
        Err(e) => {
            eprintln!("[*] Error during repository removal: {}", e);
            exit(1);
        }
        _ => {
            println!("[*] Repository was removed successfully!");
            exit(0);
        }
    }
}

// Custom clap completer because identifiers are nice to have complete on
fn identifier_clap_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let mut completions = vec![];
    let Some(current) = current.to_str() else {
        return completions;
    };

    let identifiers = match App::identifiers(&dirs::config_dir().unwrap().join("wpctl")) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error when completing: {e}");
            return completions;
        }
    };

    for id in identifiers {
        if id.contains(current) {
            completions.push(CompletionCandidate::new(id));
        }
    }
    completions
}
