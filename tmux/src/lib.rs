#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub use api::{Tmux, TmuxCliApi};
pub use env::var;
pub use error::Error;

mod api;
pub mod conf;
mod env;
mod error;
pub mod tpm;
pub mod window_options;
