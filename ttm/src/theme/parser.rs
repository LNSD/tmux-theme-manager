use thiserror::Error;

use crate::theme::model::Theme;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to parse theme file")]
    ParseError(#[from] toml::de::Error),
}

pub fn parse(buf: &str) -> Result<Theme, ParserError> {
    toml::from_str(&buf).map_err(ParserError::ParseError)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{parse, ParserError};

    #[test]
    fn parse_test() {
        let theme = indoc! { r##"
            version = 1

            [window_options]
            status-fg = "white"
            status-bg = "colour7"

            status-left = "#[bg=colour240,fg=white] #S #[fg=colour236,reverse]"
            status-left-length = "40"
        "## };

        let loaded = parse(&theme).unwrap();

        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.window_options.len(), 4);
        assert_eq!(loaded.window_options["status-bg"], "colour7");
    }

    #[test]
    fn parse_error_test() {
        let theme = indoc! { r##"
            version = "X"  # This value is not valid. Version must be a positive integer

            [window_options]
            status-fg = "white"
            status-bg = "colour7"

            status-left = "#[bg=colour240,fg=white] #S #[fg=colour236,reverse]"
            status-left-length = "40"
        "## };

        let res = parse(&theme);

        assert_matches!(res, Err(ParserError::ParseError { .. }));
    }
}
