use std::{error::Error, process};

#[cfg_attr(debug_assertions, track_caller)]
pub fn unwrap_or_exit<T, E: Error>(result: Result<T, E>, code: i32) -> T {
    match result {
        Ok(x) => x,
        Err(e) => {
            #[cfg(not(debug_assertions))]
            eprintln!("{e}");

            #[cfg(debug_assertions)]
            eprintln!("{}: {e:?}", std::panic::Location::caller());

            process::exit(code)
        }
    }
}
