#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Tmux executable not found")]
    ExecutableNotFound(std::io::Error),

    #[error("Tmux command exited with non-zero status code: {0}")]
    CommandExitStatusError(i32),

    #[error("An error occurred while parsing command's output")]
    CommandOutputParsingError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
