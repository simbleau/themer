use serde::{Deserialize, Serialize};

use super::{Theme, DARK_THEME, LIGHT_THEME};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPreference {
    Dark,
    Light,
}
