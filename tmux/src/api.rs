use std::io::ErrorKind;
use std::process::Command;

use crate::error::Error;

pub trait Tmux {
    fn version(&self) -> anyhow::Result<String, Error>;
    fn show_options(&self) -> anyhow::Result<String, Error>;
    fn set_window_option(&self, option: &str, value: &str) -> anyhow::Result<(), Error>;
}

pub struct TmuxCliApi {
    executable: &'static str,
}

impl Default for TmuxCliApi {
    fn default() -> Self {
        Self { executable: "tmux" }
    }
}

impl Tmux for TmuxCliApi {
    fn version(&self) -> anyhow::Result<String, Error> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("-V");

        log::debug!("{:?}", cmd);

        let output = cmd.output().map_err(|err| match err.kind() {
            ErrorKind::NotFound => Error::ExecutableNotFound(err),
            _ => Error::IOError(err),
        })?;

        if !output.status.success() {
            let rc = output.status.code().unwrap_or(-1);
            return Err(Error::CommandExitStatusError(rc))?;
        }

        // Process stdout
        let output =
            String::from_utf8(output.stdout).map_err(|_| Error::CommandOutputParsingError)?;
        Ok(output)
    }

    fn show_options(&self) -> anyhow::Result<String, Error> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("show-options");
        cmd.arg("-gq");

        log::debug!("{:?}", cmd);

        let output = cmd.output().map_err(|err| match err.kind() {
            ErrorKind::NotFound => Error::ExecutableNotFound(err),
            _ => Error::IOError(err),
        })?;

        if !output.status.success() {
            let rc = output.status.code().unwrap_or(-1);
            return Err(Error::CommandExitStatusError(rc))?;
        }

        // Process stdout
        let output =
            String::from_utf8(output.stdout).map_err(|_| Error::CommandOutputParsingError)?;
        Ok(output)
    }

    fn set_window_option(&self, name: &str, value: &str) -> anyhow::Result<(), Error> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("set-window-option");
        cmd.arg("-g");
        cmd.arg(name);
        cmd.arg(value);

        log::debug!("{:?}", cmd);

        let output = cmd.output().map_err(|err| match err.kind() {
            ErrorKind::NotFound => Error::ExecutableNotFound(err),
            _ => Error::IOError(err),
        })?;

        if !output.status.success() {
            let rc = output.status.code().unwrap_or(-1);
            return Err(Error::CommandExitStatusError(rc))?;
        }

        Ok(())
    }
}
