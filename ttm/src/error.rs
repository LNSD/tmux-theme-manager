use crate::theme;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Theme file not found")]
    ThemeFileNotFound(std::io::Error),

    #[error("An error occurred while opening the theme file")]
    ThemeFileOpenError(std::io::Error),

    #[error("Theme file read error")]
    ThemeFileReadError(std::io::Error),

    #[error("An error occurred while parsing the theme file")]
    ThemeParsingError(#[from] theme::parser::ParserError),

    #[error(transparent)]
    TmuxError(#[from] tmux::Error),
}
