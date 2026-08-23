use serde::{Deserialize, Serialize};

/// Whether a bluetooth device is currently connected.
///
/// Windows reports several distinct "not connected" states, none of which mean anything different
/// to the app, so they all collapse into [`ConnectionState::Disconnected`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

impl ConnectionState {
    pub const fn is_connected(self) -> bool {
        matches!(self, Self::Connected)
    }
}
