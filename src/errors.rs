use std::{error::Error, process};

/// Unwraps a `Result`, exiting the process with a given code on error.
///
/// This is similar to `Result::unwrap`, but instead of panicking, it prints the error
/// and exits the process with the provided exit code so the end-user can understand whato do.
///
/// - In **debug builds**, it prints the caller location and the error using `Debug`.
/// - In **release builds**, it prints only the error using `Display`.
///
/// # Arguments
/// * `result` - The `Result<T, E>` to unwrap.
/// * `code` - Exit code to use if the `Result` is `Err`.
///
/// # Returns
/// The value `T` if the `Result` is `Ok`.
#[cfg_attr(debug_assertions, track_caller)]
pub fn unwrap_or_exit<T, E: Error>(result: Result<T, E>, code: i32) -> T {
    match result {
        Ok(x) => x,
        Err(e) => {
            #[cfg(not(debug_assertions))]
            eprintln!(
                "An error occured while trying to unwrap a value: {e}\n\nPlease report this to the GitHub repository issues"
            );

            #[cfg(debug_assertions)]
            eprintln!("{}: {e:?}", std::panic::Location::caller());

            process::exit(code)
        }
    }
}
