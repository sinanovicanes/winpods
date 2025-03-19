type AirPodsModel =
  | "AirPods1"
  | "AirPods2"
  | "AirPods3"
  | "AirPodsPro"
  | "AirPodsPro2"
  | "AirPodsPro2UsbC"
  | "AirPodsMax"
  | "BeatsFitPro"
  | "Unknown";

interface Battery {
  level: number;
  charging: boolean;
}

interface DeviceProperties {
  rssi: number;
  address: number;
  model: AirPodsModel;
  leftBattery: Battery;
  rightBattery: Battery;
  caseBattery?: Battery;
}

interface Device {
  address: number;
  name: string;
  connectionState: "connected" | "disconnected";
}
