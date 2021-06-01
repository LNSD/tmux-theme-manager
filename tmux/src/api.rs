use crate::error::TmuxError;

// #[cfg_attr(test, mockall::automock)]
pub trait Tmux {
    fn version(&self) -> anyhow::Result<String, TmuxError>;
    fn show_options(&self) -> anyhow::Result<String, TmuxError>;
    fn set_window_option(&self, option: &str, value: &str) -> anyhow::Result<(), TmuxError>;
}
