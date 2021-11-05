use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmuxError {
    #[error("Executable not found")]
    ExecutableNotFound { src: std::io::Error },

    #[error("Tmux command filed: {0}")]
    ExitStatusError(i32),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
