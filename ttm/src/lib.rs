#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[cfg(test)]
#[macro_use]
extern crate maplit;

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

use tmux::api::Tmux;
use tmux::conf::{conf_dirs, home_dir, xdg_config_dir};
use tmux::plugins::tpm;

use crate::error::TtmError;
use crate::theme::model::Theme;

pub mod error;
pub mod theme;

#[deprecated = "TODO: Move to the right module"]
pub fn set_theme<T: Tmux>(tmux: &T, theme: &Theme) -> anyhow::Result<(), TtmError> {
    for (key, value) in theme.iter() {
        tmux.set_window_option(key, value)
            .map_err(|e| TtmError::TmuxRunError(e))?;
    }

    Ok(())
}

#[deprecated = "TODO: Move to the right module"]
const THEME_FILE_EXTENSION: &str = "ttm";

#[deprecated = "TODO: Move to the right module"]
fn get_theme_plugin_file_path(theme: &str) -> PathBuf {
    let mut file_path = PathBuf::from(theme);

    let ext = file_path.extension();
    if ext.is_none() {
        file_path.set_extension(THEME_FILE_EXTENSION);
    }

    file_path
}

#[deprecated = "TODO: Move to the right module"]
pub fn get_theme_path(theme: &str) -> Option<PathBuf> {
    let home = home_dir();
    let xdg_config = xdg_config_dir(&home);

    conf_dirs(&home, &xdg_config)
        .iter()
        .find(|&p| p.is_dir())
        .map(|p| tpm::plugins_dir(p))
        .map(|plugins_dir| plugins_dir.join(get_theme_plugin_file_path(theme)))
}

#[deprecated = "TODO: Move to the right module"]
pub fn load_theme(theme_path: &Path) -> Result<Theme, TtmError> {
    let mut theme_file = File::open(theme_path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => TtmError::ThemeFileNotFound(e),
        _ => TtmError::UnknownError("An unknown error occurred while opening the theme file"),
    })?;

    let mut buf = String::new();
    theme_file
        .read_to_string(&mut buf)
        .map_err(TtmError::ThemeFileReadError)?;

    theme::parser::parse(&buf).map_err(TtmError::ThemeParsingError)
}

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;
//
//     use mockall::predicate::*;
//
//     use crate::tmux::wrapper::*;
//
//     use super::{get_config, resolve_theme_path};
//
//     #[test]
//     fn get_config_test() {
//         let mut tmux_wrapper = MockTmuxWrapper::new();
//         tmux_wrapper
//             .expect_list_options()
//             .returning(|| Ok(vec!["@theme 'papa'".to_string()]));
//
//         let config = get_config(Box::new(tmux_wrapper)).unwrap();
//         assert_eq!(config.theme, "'papa'");
//     }
//
//     #[test]
//     fn resolve_theme_path_test() {
//         let theme = "some/theme";
//         let plugins_dir = PathBuf::from("/tpm/plugins/");
//         let path = resolve_theme_path(&plugins_dir, theme).unwrap();
//
//         assert_eq!(path.to_str().unwrap(), "/tpm/plugins/some/theme.ttm")
//     }
// }
