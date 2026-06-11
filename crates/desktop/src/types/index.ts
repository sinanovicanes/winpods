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
  updatedAtUnixMs: number;
  model: AirPodsModel;
  leftBattery: Battery;
  rightBattery: Battery;
  caseBattery?: Battery;
}

type DeviceConnectionState = "connected" | "disconnected";

interface Device {
  address: number;
  name: string;
  connectionState: DeviceConnectionState;
  model: AirPodsModel;
}

interface ServiceToggleSummary {
  attempted: number;
  succeeded: number;
  failed: number;
}

interface AudioEndpoint {
  id: string;
  name: string;
  isDefault: boolean;
  isAirpodsMatch: boolean;
}

interface AudioRouteSummary {
  matchedEndpoint?: AudioEndpoint;
  endpoints: AudioEndpoint[];
  rolesAttempted: number;
  rolesSucceeded: number;
  rolesFailed: number;
}
