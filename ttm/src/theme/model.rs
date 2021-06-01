use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub version: i32,
    pub window_options: HashMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            version: 0,
            window_options: Default::default(),
        }
    }
}

impl From<HashMap<String, String>> for Theme {
    fn from(options: HashMap<String, String>) -> Self {
        Self {
            version: 0,
            window_options: options,
        }
    }
}

impl Theme {
    pub fn update(&mut self, theme_ref: &HashMap<String, String>) {
        self.window_options
            .extend(theme_ref.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.window_options.iter()
    }

    pub fn into_iter(self) -> IntoIter<String, String> {
        self.window_options.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Theme;

    #[test]
    fn theme_update_test() {
        let mut theme = Theme {
            version: 1,
            window_options: HashMap::new(),
        };
        let extra_opts = hashmap! {
            String::from("option-1") => String::from("value1"),
            String::from("option-2") => String::from("value2"),
        };

        theme.update(&extra_opts);

        assert_eq!(theme.window_options["option-1"], extra_opts["option-1"])
    }
}
