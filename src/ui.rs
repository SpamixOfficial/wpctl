use cursive::{views::{Dialog, TextView}, CursiveRunnable};
use std::{env::args, process::exit};

pub fn main() {
    // Handle all of our possible arguments
    handle_args();

    // Creates the cursive root - required for every application.
    let mut siv: CursiveRunnable = cursive::default();

    siv.add_global_callback('q', |s| s.quit());    

    // Creates a dialog with a single "Quit" button
    siv.add_layer(TextView::new("Hello Dialog!"));

    // Starts the event loop.
    siv.run();
}
