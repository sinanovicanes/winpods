use device::managers::DeviceManager;

#[tokio::main]
async fn main() {
    let mut device_manager = DeviceManager::new();
    let _ = device_manager.scan();

    println!("{:?}", device_manager);
}
