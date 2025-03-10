use bluetooth::AdvirtesementWatcher;
use device::apple_cp;
use tokio::signal;
use windows::core::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut watcher = AdvirtesementWatcher::new();

    watcher.on_received(|data| {
        apple_cp::analyze_airpods_data(&data.manufacturer_data_map.get(&76).unwrap());
    });

    watcher.start();

    // Keep running until Ctrl+C is pressed
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    watcher.stop();

    Ok(())
}
