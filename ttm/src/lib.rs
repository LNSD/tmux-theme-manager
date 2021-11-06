#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[cfg(test)]
#[macro_use]
extern crate maplit;

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

use tmux::Tmux;

pub use crate::error::Error;
pub use crate::theme::model::Theme;

mod error;
pub mod theme;

pub fn load_theme(theme_path: &Path) -> anyhow::Result<Theme, Error> {
    let mut theme_file = File::open(theme_path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => Error::ThemeFileNotFound(e),
        _ => Error::ThemeFileOpenError(e),
    })?;

    let mut buf = String::new();
    theme_file
        .read_to_string(&mut buf)
        .map_err(Error::ThemeFileReadError)?;

    theme::parser::parse(&buf).map_err(Error::ThemeParsingError)
}

pub fn set_theme<T: Tmux>(tmux: &T, theme: &Theme) -> anyhow::Result<(), Error> {
    for (key, value) in theme.iter() {
        tmux.set_window_option(key, value)
            .map_err(|e| Error::TmuxError(e))?;
    }
    Ok(())
}
