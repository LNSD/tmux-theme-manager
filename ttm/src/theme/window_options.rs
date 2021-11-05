use std::collections::HashMap;

use lazy_regex::{Lazy, lazy_regex, Regex};

static THEME_OPTION: &str = "@theme";
static THEME_OPTION_NAME_REGEX: Lazy<Regex> = lazy_regex!(r"@theme-(?P<name>[\w\-\[\]].*?)$");

pub fn get_theme(options: &HashMap<&str, &str>) -> Option<String> {
    options.get(THEME_OPTION).map(|&s| s.to_owned())
}

pub fn get_theme_window_options(options: &HashMap<&str, &str>) -> HashMap<String, String> {
    options
        .iter()
        .filter(|(&key, _)| THEME_OPTION_NAME_REGEX.is_match(key))
        .map(|(&key, &value)| {
            let name = THEME_OPTION_NAME_REGEX
                .captures(key)
                .unwrap()
                .name("name")
                .unwrap()
                .as_str();
            (name.to_owned(), value.to_owned())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use tmux::window_options::parse_window_options;

    use super::{get_theme, get_theme_window_options, THEME_OPTION_NAME_REGEX};

    static RAW_TEST_OPTIONS: &str = indoc! { r##"
        @theme tmux-powerline-theme/powerline
        @theme-show-session 1
        @theme-show-session 2
        @theme-flag true
        @theme tmux-powerline-theme/powerline3k
        @theme-show-session 3
        renumber-windows on
        repeat-time 500
        status on
        status-format[0] "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]"
        status-format[1] "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
        status-justify left
        status-keys emacs
        status-left "#[bg=colour240,fg=white] #S #[fg=colour236,reverse]"
        lock-command "lock -np"
    "## };

    #[test]
    fn theme_option_name_regex_non_matching_test() {
        let option_name = "@theme";
        assert!(!THEME_OPTION_NAME_REGEX.is_match(option_name));
    }

    #[test]
    fn theme_option_name_regex_match_test() {
        let option_name = "@theme-show-session[0]";
        let name = THEME_OPTION_NAME_REGEX
            .captures(option_name)
            .unwrap()
            .name("name")
            .unwrap()
            .as_str();

        assert_eq!(name, "show-session[0]");
    }

    #[test]
    fn get_theme_test() {
        let window_options = parse_window_options(RAW_TEST_OPTIONS);
        let theme = get_theme(&window_options);

        assert_eq!(theme.unwrap(), "tmux-powerline-theme/powerline3k");
    }

    #[test]
    fn get_theme_window_options_test() {
        let window_options = parse_window_options(RAW_TEST_OPTIONS);
        let theme_options = get_theme_window_options(&window_options);

        assert_eq!(theme_options.len(), 2);
        assert_eq!(theme_options["flag"], "true");
        assert_eq!(theme_options["show-session"], "3");
    }
}
