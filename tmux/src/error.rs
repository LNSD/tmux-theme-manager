use thiserror::Error;

use crate::env;

#[derive(Error, Debug)]
pub enum TmuxError {
    #[error("Executable not found")]
    ExecutableNotFound { src: std::io::Error },

    /// Represents all cases of `crate::socket::SocketError`.
    #[error(transparent)]
    SocketError(#[from] env::EnvironError),

    #[error("Tmux command filed: {0}")]
    ExitStatusError(i32),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
