use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Error {
    DeviceNotFound,
    PropertyNotFound,
    WindowsError,
}
