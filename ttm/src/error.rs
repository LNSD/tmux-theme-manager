use thiserror::Error;

use crate::theme;

#[derive(Error, Debug)]
pub enum TtmError {
    #[error("En error occurred while running a TMUX command")]
    TmuxRunError(#[from] tmux::error::TmuxError),

    #[error("Theme file not found")]
    ThemeFileNotFound(std::io::Error),

    #[error("Theme file not found")]
    ThemeFileReadError(std::io::Error),

    #[error("An error occurred while parsing the theme file")]
    ThemeParsingError(#[from] theme::parser::ParserError),

    #[deprecated]
    #[error("Unknown error")]
    UnknownError(&'static str),
}
