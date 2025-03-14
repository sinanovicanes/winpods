use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    pub level: u8,
    pub charging: bool,
}

impl Battery {
    pub fn new(level: u8, charging: bool) -> Self {
        Self { level, charging }
    }
}
