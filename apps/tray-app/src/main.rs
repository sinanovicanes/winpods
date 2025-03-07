use tokio::signal;
use windows::{
    core::{Error, Ref},
    Devices::Bluetooth::Advertisement::{
        BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
        BluetoothLEScanningMode,
    },
    Foundation::TypedEventHandler,
};

#[tokio::main]
async fn main() -> windows::core::Result<()> {
    let watcher = BluetoothLEAdvertisementWatcher::new()?;
    watcher.SetScanningMode(BluetoothLEScanningMode::Active)?;

    watcher.Received(&TypedEventHandler::<
        BluetoothLEAdvertisementWatcher,
        BluetoothLEAdvertisementReceivedEventArgs,
    >::new(|_watcher, args| {
        println!("Received advertisement.");
        let device = args.as_ref().ok_or(Error::empty())?;
        let address = device.BluetoothAddress()?;
        let manufacturer_data = device.Advertisement()?.ManufacturerData()?;

        for data in manufacturer_data {
            let company_id = data.CompanyId()?;
            let data = data.Data()?;

            println!("Company ID: {}", company_id);
            println!("Data: {:?}", data);
        }

        println!("Address: {}", address);
        Ok(())
    }))?;

    watcher.Start()?;
    println!("Scanning for Bluetooth LE advertisements...");

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    watcher.Stop()?;

    println!("Stopped scanning.");
    Ok(())
}
