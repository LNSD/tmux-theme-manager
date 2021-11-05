use std::collections::HashMap;

fn parse_window_option_item(raw: &str) -> (&str, &str) {
    let mut splitter = raw.splitn(2, ' ');
    let key = splitter.next().unwrap();
    let value = splitter.next().unwrap();
    (key, value)
}

pub fn parse_window_options(raw: &str) -> HashMap<&str, &str> {
    raw.lines()
        .into_iter()
        .map(parse_window_option_item)
        .collect()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{parse_window_option_item, parse_window_options};

    #[test]
    fn parse_window_option_test() {
        let raw_option = "word-separators \" \"";
        let (key, value) = parse_window_option_item(raw_option);

        assert_eq!(key, "word-separators");
        assert_eq!(value, "\" \"");
    }

    #[test]
    fn parse_window_options_vector_test() {
        let raw_opts: &str = indoc! { r##"
            @theme tmux-powerline-theme/powerline
            @theme-show-session 1
            @theme-show-session 2
            @theme-flag true
            @theme tmux-powerline-theme/powerline
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

        let options = parse_window_options(raw_opts);
        assert_eq!(options.len(), 12);
        assert_eq!(options["@theme-show-session"], "3");
        assert_eq!(options["status-justify"], "left");
        assert_eq!(options["lock-command"], "\"lock -np\"");
    }
}
