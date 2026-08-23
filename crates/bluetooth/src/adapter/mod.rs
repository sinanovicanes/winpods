//! Access to the machine's bluetooth radio.

mod state;
mod watcher;

pub use state::AdapterState;
pub use watcher::AdapterWatcher;

use anyhow::{Context, Result};
use tokio::sync::OnceCell;
use windows::Devices::{
    Bluetooth::BluetoothAdapter,
    Radios::{Radio, RadioAccessStatus, RadioKind},
};

/// Radio access only has to be requested once per process.
static RADIO_ACCESS: OnceCell<()> = OnceCell::const_new();

/// Asks Windows for permission to use this machine's radios.
///
/// Required once per process before the radio APIs return anything useful; depending on the
/// Windows build and the user's privacy settings, skipping it makes the machine look like it has
/// no radios at all. A denial is logged rather than raised, because the fallbacks below may
/// still work and the app degrades to "bluetooth is off".
async fn request_radio_access() {
    RADIO_ACCESS
        .get_or_init(|| async {
            let status = match Radio::RequestAccessAsync() {
                Ok(operation) => operation.await,
                Err(e) => Err(e),
            };

            match status {
                Ok(status) if status == RadioAccessStatus::Allowed => {
                    tracing::debug!("Radio access granted");
                }
                Ok(status) => tracing::warn!("Radio access was not granted: {status:?}"),
                Err(e) => tracing::warn!("Failed to request radio access: {e}"),
            }
        })
        .await;
}

/// Returns the radio backing the default bluetooth adapter.
///
/// `BluetoothAdapter::GetRadioAsync` is unreliable outside packaged apps: it can fail with
/// `REGDB_E_CLASSNOTREG`, and it reports nothing when the process architecture does not match the
/// machine's (an x64 build running emulated on an ARM64 device, for instance). Enumerating the
/// radios directly is the fallback.
pub async fn bluetooth_radio() -> Result<Radio> {
    crate::com::ensure_mta();
    request_radio_access().await;

    match default_adapter_radio().await {
        Ok(radio) => return Ok(radio),
        Err(e) => tracing::warn!(
            "Could not get the radio of the default bluetooth adapter: {e:#}. \
             Falling back to enumerating radios"
        ),
    }

    bluetooth_radios()
        .await?
        .into_iter()
        .next()
        .context("no bluetooth radio is available on this machine")
}

async fn default_adapter_radio() -> Result<Radio> {
    let adapter = BluetoothAdapter::GetDefaultAsync()?
        .await
        .context("no default bluetooth adapter")?;

    adapter
        .GetRadioAsync()?
        .await
        .context("the default bluetooth adapter has no radio")
}

/// Returns every bluetooth radio on this machine.
pub async fn bluetooth_radios() -> Result<Vec<Radio>> {
    crate::com::ensure_mta();
    request_radio_access().await;

    let radios = Radio::GetRadiosAsync()?
        .await
        .context("failed to enumerate this machine's radios")?;

    Ok(radios
        .into_iter()
        .filter(|radio| radio.Kind().is_ok_and(|kind| kind == RadioKind::Bluetooth))
        .collect())
}

/// Reads the current adapter state, treating an unreachable radio as "off".
pub async fn adapter_state() -> AdapterState {
    match bluetooth_radio().await {
        Ok(radio) => AdapterState::from(&radio),
        Err(e) => {
            tracing::warn!("Could not read the bluetooth adapter state: {e:#}");
            AdapterState::Off
        }
    }
}
