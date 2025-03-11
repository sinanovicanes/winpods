use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    level: u8,
    charging: bool,
}

impl Battery {
    pub fn new(level: u8, charging: bool) -> Self {
        Self { level, charging }
    }
}
