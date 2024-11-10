use std::{any::Any, thread::sleep, time::Duration};

/// Various utilities, like error handler functions

/// Handles potential error and can pass OK value through callback
/// Use ecallback for callback on error. Ran before print
pub fn ehandle<T>(rs: anyhow::Result<T>, callback: Option<fn(T) -> T>, ecallback: Option<fn(anyhow::Error) -> anyhow::Error>) -> Option<T>
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
,
    }
}
