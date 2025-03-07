use bluetooth::get_bluetooth_device;
use device::models::Airpods;

#[tokio::main]
async fn main() {
    let device = get_bluetooth_device().await.unwrap();
    let airpods = Airpods::try_from(device).unwrap();

    println!("Airpods: {:?}", airpods);
}
