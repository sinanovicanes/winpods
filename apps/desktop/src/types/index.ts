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

interface ConnectedDevice {
  name: string;
  model: AirPodsModel;
  rightBattery: Battery;
  leftBattery: Battery;
  caseBattery?: Battery;
}
