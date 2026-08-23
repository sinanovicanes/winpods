use std::sync::Once;

use windows::Win32::System::Com::CoIncrementMTAUsage;

static MTA: Once = Once::new();

/// Registers an implicit multithreaded apartment (MTA) for the whole process.
///
/// Every WinRT call has to run inside a COM apartment. Tokio worker threads never initialize
/// one, so without this the first WinRT call made from an async task fails with
/// `CO_E_NOTINITIALIZED` -- which is exactly what happens once the blocking `.get()` calls are
/// replaced by `.await`.
///
/// `CoIncrementMTAUsage` marks the process as having an implicit MTA that every thread which did
/// not pick an apartment of its own joins automatically. The returned cookie is a plain handle
/// that is never passed to `CoDecrementMTAUsage`: the apartment has to outlive every WinRT object
/// this crate hands out, which in practice means the whole process.
pub fn ensure_mta() {
    MTA.call_once(|| {
        // SAFETY: `CoIncrementMTAUsage` takes no arguments and only returns a cookie. Leaking
        // the cookie is intentional -- dropping it would tear the apartment down while WinRT
        // objects owned by the app are still alive.
        #[allow(unsafe_code)]
        let result = unsafe { CoIncrementMTAUsage() };

        match result {
            // The cookie is intentionally dropped without decrementing: the implicit MTA is
            // meant to last for the lifetime of the process.
            Ok(_cookie) => tracing::debug!("Registered the implicit COM MTA for this process"),
            Err(e) => tracing::error!("Failed to register the implicit COM MTA: {e}"),
        }
    });
}
