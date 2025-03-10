use bluetooth::AdvirtesementWatcher;
use tokio::signal;
use windows::core::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut watcher = AdvirtesementWatcher::new();

    watcher.on_received(|data| {
        println!("{:?}", data);
    });

    watcher.start();

    // Keep running until Ctrl+C is pressed
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    watcher.stop();

    Ok(())
}
