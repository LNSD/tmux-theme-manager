use std::{io::ErrorKind, process::Command};

use crate::api::Tmux;
use crate::error::TmuxError;

#[derive(Debug)]
pub struct TmuxCli {
    executable: &'static str,
}

impl Default for TmuxCli {
    fn default() -> Self {
        Self { executable: "tmux" }
    }
}

impl Tmux for TmuxCli {
    fn version(&self) -> anyhow::Result<String, TmuxError> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("-V");

        log::trace!("{:?}", cmd);

        let out = cmd.output().map_err(|err| match err.kind() {
            ErrorKind::NotFound => TmuxError::ExecutableNotFound { src: err },
            _ => TmuxError::IOError(err),
        })?;

        let version = String::from_utf8_lossy(&out.stdout).into_owned();
        Ok(version)
    }

    fn show_options(&self) -> anyhow::Result<String, TmuxError> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("show-options");
        cmd.arg("-gq");

        log::trace!("{:?}", cmd);

        let out = cmd.output().map_err(|err| match err.kind() {
            ErrorKind::NotFound => TmuxError::ExecutableNotFound { src: err },
            _ => TmuxError::IOError(err),
        })?;

        // Process stdout
        let options = String::from_utf8_lossy(&out.stdout);
        Ok(options.into_owned())
    }

    fn set_window_option(&self, name: &str, value: &str) -> anyhow::Result<(), TmuxError> {
        let mut cmd = Command::new(self.executable);
        cmd.arg("set-window-option");
        cmd.arg("-g");
        cmd.arg(name);
        cmd.arg(value);

        log::trace!("{:?}", cmd);

        let rc = cmd.status().map_err(|err| match err.kind() {
            ErrorKind::NotFound => TmuxError::ExecutableNotFound { src: err },
            _ => TmuxError::IOError(err),
        })?;

        if !rc.success() {
            return match rc.code() {
                Some(code) => Err(TmuxError::ExitStatusError(code)),
                None => Err(TmuxError::ExitStatusError(-1)),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Tmux;
    use crate::error::TmuxError;

    use super::TmuxCli;

    #[test]
    fn get_version_test() {
        let wrapper = TmuxCli::default();

        let version = match wrapper.version() {
            Ok(version) => version,
            Err(err) => panic!("{:?}", err),
        };

        assert_eq!(version.trim(), "tmux 3.2a")
    }

    #[test]
    fn get_version_bad_executable_test() {
        let wrapper = TmuxCli {
            executable: "not-tmux",
        };

        let error = wrapper.version().unwrap_err();
        assert_matches!(error, TmuxError::ExecutableNotFound { .. })
    }

    #[ignore]
    #[test]
    fn list_options_test() {
        let wrapper = TmuxCli::default();
        let opts = wrapper.show_options().unwrap();
        assert!(!opts.is_empty());
    }
}
