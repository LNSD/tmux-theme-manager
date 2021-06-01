use std::path::{Path, PathBuf};

pub fn plugins_dir(tmux_dir: &Path) -> PathBuf {
    tmux_dir.join("plugins/")
}

fn resolve_plugin_dir(tmux_dir: &Path, plugin_name: &str) -> PathBuf {
    plugins_dir(tmux_dir).join(plugin_name)
}

pub fn is_installed(tmux_dir: &Path) -> bool {
    let tpm_plugins_dir = plugins_dir(tmux_dir);
    if !tpm_plugins_dir.is_dir() {
        return false;
    }

    let tpm_dir = resolve_plugin_dir(tmux_dir, "tpm/");
    if !tpm_dir.is_dir() {
        return false;
    }

    let tpm_exec = tpm_dir.join("tpm");
    tpm_exec.is_file()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{plugins_dir, resolve_plugin_dir};

    #[test]
    fn resolve_plugins_dir_test() {
        let tmux_conf_dir_path = PathBuf::from("/root/.tmux/");
        let plugins_dir_path = plugins_dir(&tmux_conf_dir_path);

        assert_eq!(plugins_dir_path.to_string_lossy(), "/root/.tmux/plugins/")
    }

    #[test]
    fn resolve_plugin_dir_test() {
        let plugin_name = "tmux-powerline-theme";
        let tmux_conf_dir_path = PathBuf::from("/root/.tmux");
        let plugins_dir_path = resolve_plugin_dir(&tmux_conf_dir_path, plugin_name);

        assert_eq!(
            plugins_dir_path.to_string_lossy(),
            format!("/root/.tmux/plugins/{}", plugin_name)
        )
    }
}
