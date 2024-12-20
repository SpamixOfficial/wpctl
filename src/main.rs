use app::App;
use file_matcher::FileNamed;
use backend::{repository::RepositoryManifest, wallpaper::WpManifest};
use clap::{Parser, Subcommand};
use clap_complete::{ArgValueCompleter, CompletionCandidate};
use regex::Regex;
use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::{copy, Cursor, Write},
    process::exit,
};
mod app;
mod backend;
mod ui;
mod utils;
// Old, early cursive TUI thingys, moving to ratatui now :-)
//mod old/ui; // Note that this file needs to be imported for any ui functions to be available!!

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version = VERSION, about = "Package manager for wallpapers", long_about = None, arg_required_else_help(true))] // Set this to false when ui is enabled!
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
    #[command(long_about = "Update selected repositories")]
    UpdateRepos {
        #[arg(add = ArgValueCompleter::new(identifier_clap_completer), help = "Package to remove, by identifier")]
        identifiers: Vec<String>,
    },
    #[command(long_about = "Update all repositories")]
    UpdateAllRepos,
    #[command(long_about = "Remove a local repository from an identifier")]
    RemoveRepo {
        #[arg(add = ArgValueCompleter::new(identifier_clap_completer), help = "Package to remove, by identifier")]
        identifier: String,
    },
    #[command(long_about = "Package handling, such as installation and searching!")]
    Packages {
        #[arg(short = 'Q', long = "query", help = "Query all packages using the provided regex", conflicts_with_all(["install", "remove"]))]
        query: Option<Regex>,
        #[arg(short = 'S', long = "install", help = "Install package(s)", conflicts_with_all(["query", "remove"]))]
        install: Option<Vec<String>>,
        #[arg(short = 'R', long = "remove", help = "Remove package(s)", conflicts_with_all(["install", "query"]))]
        remove: Option<Vec<String>>,
    },
    #[command(long_about = "Set the wallpaper using this command")]
    Set {
        package: String
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

    // UI will be enabled later, but at the moment it is not ready really....
    /*match ui::run(app) {
        Err(e) => {
            eprintln!(
                "Uh oh! Something went wrong with the ui. Exiting...\n{}",
                e.to_string()
            );
            exit(1)
        }
        _ => (),
    }*/
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
            Command::UpdateRepos { identifiers } => command_update_repos(app, identifiers),
            Command::UpdateAllRepos => command_update_all_repos(app),
            Command::Set { package } => command_set_wallpaper(app, package),
            Command::Packages {
                query,
                install,
                remove,
            } => {
                if let Some(q) = query {
                    command_query_packages(app, q);
                } else if let Some(pkgs) = install {
                    command_install_packages(app, pkgs);
                } else if let Some(pkgs) = remove {
                    command_remove_packages(app, pkgs);
                }
            }
        };
    }
}

fn command_query_packages(app: &mut App, query: Regex) {
    let packages: Vec<(WpManifest, RepositoryManifest)> = app
        .wp_items
        .iter()
        .filter(|(f, _)| query.is_match(f.name.as_str()) || query.is_match(f.description.as_str()))
        .map(|f| f.to_owned())
        .collect();

    packages.iter().for_each(|(f, r)| {
        println!(
            "{}/{}:\n\tDescription: {}\n\tThumbnail: {}",
            r.identifier, f.name, f.description, f.thumbnail_url
        )
    });
}

fn command_set_wallpaper(app: &mut App, package: String) {
    let path = app.approot.join("packages").join(&package);
    if !path.try_exists().unwrap_or(false) {
        eprintln!("[*] No such package installed: {}", &package);
        exit(1);
    };
    let file = FileNamed::wildmatch("img.*").within(&path).find().unwrap();
    wallpaper::set_from_path(file.to_str().unwrap()).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Center).unwrap();
    println!("[*] Wallpaper was successfully set!")
}

fn command_remove_packages(app: &mut App, packages: Vec<String>) {
    for pkg in packages {
        let path = app.approot.join("packages").join(&pkg);
        if !path.try_exists().unwrap_or(false) {
            eprintln!("[*] No such package installed: {}", &pkg);
            exit(1);
        };
        remove_dir_all(&path).unwrap();
        println!("[*] Package {} was successfully removed!", pkg);
    } 
}

fn command_install_packages(app: &mut App, packages: Vec<String>) {
    // We use the repo_id/pak_id for all packages, since it makes organizing and finding packages
    // way easier.
    let mut install_packages: Vec<(WpManifest, String)> = vec![];
    let formatted_wp_items: Vec<(WpManifest, String)> = app
        .wp_items
        .iter()
        .map(|f| (f.0.to_owned(), format!("{}/{}", f.1.identifier, f.0.id)))
        .collect();

    for pkg in packages {
        if !formatted_wp_items.iter().any(|(_, f)| f == &pkg) {
            eprintln!("[*] No such package: {}", pkg);
            exit(1);
        }
        install_packages.push(
            formatted_wp_items
                .iter()
                .find(|(_, f)| f == &pkg)
                .unwrap()
                .to_owned(),
        );
    }

    for (manifest, path) in install_packages {
        create_dir_all(app.approot.join("packages").join(&path)).unwrap();
        let mut file = File::create(app.approot.join("packages").join(&path).join("manifest.toml")).unwrap();
        file.write_all(toml::to_string(&manifest).unwrap().as_bytes())
            .unwrap();

        println!("[*] Downloading wallpaper...");
        let client = reqwest::blocking::Client::new();
        let resp = match client.get(&manifest.download_url).send() {
            Err(e) => {
                eprintln!("[*] Failed to download: {}", e);
                exit(1);
            }
            Ok(x) => x,
        };

        let url = url::Url::parse(&manifest.download_url).unwrap();
        let file_ext = std::path::Path::new(url.path_segments().unwrap().last().unwrap()).extension().unwrap().to_str().unwrap();
        let mut img_file =
            File::create(app.approot.join("packages").join(&path).join(format!("img.{}", file_ext))).unwrap();
        copy(&mut Cursor::new(resp.bytes().unwrap()), &mut img_file).unwrap();

        println!("[*] Package {} was installed successfully!", path);
    }

    exit(0);
}

fn command_update_all_repos(app: &mut App) {
    match app.all_update_repo() {
        Err(e) => {
            eprintln!("[*] Error during updates of repositories: {}", e);
            exit(1);
        }
        Ok(_) => {
            println!("[*] Updates were successful!");
            exit(0);
        }
    }
}

fn command_update_repos(app: &mut App, ids: Vec<String>) {
    let manifests: Vec<RepositoryManifest> = app
        .repositories
        .iter()
        .filter(|x| ids.contains(&x.identifier))
        .map(|x| x.to_owned())
        .collect();

    if manifests.len() == 0 {
        eprintln!("[*] No such identifiers");
        exit(1);
    }

    for manifest in manifests {
        match app.update_repo(manifest.clone()) {
            Err(e) => {
                eprintln!("[*] Error during updates of repositories: {}", e);
                exit(1);
            }
            Ok(_) => {
                println!("[*] Update of {} was successful!", manifest.name);
                exit(0);
            }
        }
    }
    println!("[*] Updates were successful!");
    exit(0);
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

    let identifiers = match App::identifiers(&dirs::config_dir().unwrap().join("wctl")) {
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
