use crate::ui::themes::style::Theme;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Clone)]
pub struct ForestConfig {
    api_key: String,
    theme: Theme,
}

impl ForestConfig {
    pub fn load() -> Result<ForestConfig, ConfyError> {
        let cfg: ForestConfig = confy::load("forest")?;

        Ok(cfg)
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn set_api_key(&mut self, key: &str) {
        self.api_key = key.to_owned();
        confy::store("forest", self).unwrap();
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        confy::store("forest", self).unwrap();
    }
}

// I know that I'm abusing this default implementation, I plan on swapping it out for something less icky
impl Default for ForestConfig {
    fn default() -> Self {
        Self::load().unwrap()
    }
}
