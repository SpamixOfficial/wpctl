use std::{
    any::Any,
    io::{self, BufRead},
    thread::sleep,
    time::Duration,
};

/// Various utilities, like error handler functions

/// Handles potential error and can pass OK value through callback
/// Use ecallback for callback on error. Ran before print
pub fn ehandle<T>(
    rs: anyhow::Result<T>,
    callback: Option<fn(T) -> T>,
    ecallback: Option<fn(anyhow::Error) -> anyhow::Error>,
) -> Option<T>
where
    T: Any,
{
    match rs {
        Ok(x) => {
            if let Some(cb) = callback {
                return Some(cb(x));
            } else {
                Some(x)
            }
        }
        Err(mut e) => {
            if let Some(ecb) = ecallback {
                e = ecb(e);
            };
            eprintln!("[*] Error: {e}");
            eprintln!("[*] Program has been put into sleep for 5 seconds. Press CTRL+C now to exit the program. \n\n!!! This error and sleep will be present until you fix the error !!!");
            sleep(Duration::from_secs(5));
            None
        }
    }
}

pub fn areusure(message: String, choices: Vec<char>, default: char) -> char {
    let stdin = io::stdin();
    let fixed_choices = choices
        .iter()
        .map(|f| {
            if f == &default {
                f.to_ascii_uppercase()
            } else {
                f.to_owned()
            }
        })
        .map(|x| x.to_string() + "/")
        .collect::<String>();
    let mut fixed_choices = fixed_choices.chars();
    fixed_choices.next_back().unwrap();
    println!("{message} [{}]", fixed_choices.as_str());
    let mut iterator = stdin.lock().lines();
    let line1: Vec<char> = iterator
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|f| f.to_ascii_lowercase())
        .collect();
    let cchar = line1.get(0).unwrap();
    if !choices.contains(cchar) {
        println!("Not a valid choice, returning default");
        return default;
    }
    return cchar.to_owned();
}
