use std::path::PathBuf;

use tmux::conf::{conf_dirs, home_dir, xdg_config_dir};
use tmux::tpm;

const FILE_EXTENSION: &str = "ttm";

fn get_theme_plugin_file_path(theme: &str) -> PathBuf {
    let mut file_path = PathBuf::from(theme);

    let ext = file_path.extension();
    if ext.is_none() {
        file_path.set_extension(FILE_EXTENSION);
    }

    file_path
}

pub fn get_theme_path(theme: &str) -> Option<PathBuf> {
    let home = home_dir();
    let xdg_config = xdg_config_dir(&home);

    conf_dirs(&home, &xdg_config)
        .iter()
        .find(|&p| p.is_dir())
        .map(|p| tpm::plugins_dir(p))
        .map(|plugins_dir| plugins_dir.join(get_theme_plugin_file_path(theme)))
}
