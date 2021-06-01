use std::env;
use std::path::{Path, PathBuf};

use dirs;

pub fn home_dir() -> PathBuf {
    dirs::home_dir().expect("Cannot detect user's $HOME dir")
}

pub fn xdg_config_dir(home: &Path) -> PathBuf {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => home.join(".config/"),
    }
}

pub fn conf_dirs(home: &Path, xdg_config: &Path) -> Vec<PathBuf> {
    let mut conf_dirs = Vec::new();

    // $XDG_CONFIG_HOME/tmux/
    let xdg_config_tmux = xdg_config.join("tmux/");
    conf_dirs.push(xdg_config_tmux);

    // $HOME/.tmux/
    let home_tmux = home.join(".tmux/");
    conf_dirs.push(home_tmux);

    conf_dirs
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::conf_dirs;

    #[test]
    fn conf_dirs_test() {
        let home_dir = PathBuf::from("/home/someone/");
        let xdg_config_dir = PathBuf::from("/home/someone/.config/");

        let conf_dirs = conf_dirs(&home_dir, &xdg_config_dir);

        assert_eq!(conf_dirs[0], PathBuf::from("/home/someone/.config/tmux/"));
        assert_eq!(conf_dirs[1], PathBuf::from("/home/someone/.tmux/"));
    }
}
