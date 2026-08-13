import AirPodBImage from "@/assets/airpod-b.png";
import AirPodProBImage from "@/assets/airpod-pro-b.png";
import AirPodImage from "@/assets/airpod.png";
import AirPodsCaseImage from "@/assets/airpods-case.png";
import AirPodsProCaseImage from "@/assets/airpods-pro-case.png";
import AirPodsMaxImage from "@/assets/airpods_max.png";
import AirPodsFImage from "@/assets/airpods.webp";
import AirPodsProFImage from "@/assets/airpods_pro_2.webp";
import AirPodsMaxFImage from "@/assets/airpods-max.png";

export interface ModelDetails {
  name: string;
  image: string;
  widget: { image: string; caseImage?: string; repeat: boolean };
}

// TODO: Add missing images
export const AirPodsModelDetails: Record<AirPodsModel, ModelDetails> = {
  ["AirPods1"]: {
    name: "AirPods 1",
    image: AirPodsFImage,
    widget: {
      image: AirPodImage,
      caseImage: AirPodsCaseImage,
      repeat: true
    }
  },
  ["AirPods2"]: {
    name: "AirPods 2",
    image: AirPodsFImage,
    widget: {
      image: AirPodBImage,
      caseImage: AirPodsCaseImage,
      repeat: true
    }
  },
  ["AirPods3"]: {
    name: "AirPods 3",
    image: AirPodsFImage,
    widget: {
      image: AirPodBImage,
      caseImage: AirPodsCaseImage,
      repeat: true
    }
  },
  ["AirPods4"]: {
    name: "AirPods 4",
    image: AirPodsFImage,
    widget: {
      image: AirPodBImage,
      caseImage: AirPodsCaseImage,
      repeat: true
    }
  },
  ["AirPods4Anc"]: {
    name: "AirPods 4 (ANC)",
    image: AirPodsFImage,
    widget: {
      image: AirPodBImage,
      caseImage: AirPodsCaseImage,
      repeat: true
    }
  },
  ["AirPodsPro"]: {
    name: "AirPods Pro",
    image: AirPodsProFImage,
    widget: {
      image: AirPodProBImage,
      caseImage: AirPodsProCaseImage,
      repeat: true
    }
  },
  ["AirPodsPro2"]: {
    name: "AirPods Pro 2",
    image: AirPodsProFImage,
    widget: {
      image: AirPodProBImage,
      caseImage: AirPodsProCaseImage,
      repeat: true
    }
  },
  ["AirPodsPro2UsbC"]: {
    name: "AirPods Pro 2 (USB-C)",
    image: AirPodsProFImage,
    widget: {
      image: AirPodProBImage,
      caseImage: AirPodsProCaseImage,
      repeat: true
    }
  },
  ["AirPodsPro3"]: {
    name: "AirPods Pro 3",
    image: AirPodsProFImage,
    widget: {
      image: AirPodProBImage,
      caseImage: AirPodsProCaseImage,
      repeat: true
    }
  },
  ["AirPodsMax"]: {
    name: "AirPods Max",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["AirPodsMaxUsbC"]: {
    name: "AirPods Max (USB-C)",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  // TODO: Beats models are using the AirPods Max images as a placeholder
  ["PowerbeatsPro"]: {
    name: "Powerbeats Pro",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["PowerbeatsPro2"]: {
    name: "Powerbeats Pro 2",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["BeatsFitPro"]: {
    name: "Beats Fit Pro",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["BeatsStudioBuds"]: {
    name: "Beats Studio Buds",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["BeatsStudioBudsPlus"]: {
    name: "Beats Studio Buds +",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["BeatsSoloBuds"]: {
    name: "Beats Solo Buds",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  },
  ["Unknown"]: {
    name: "Unknown",
    image: AirPodsMaxFImage,
    widget: { image: AirPodsMaxImage, repeat: false }
  }
};

export function getModelDetails(model: AirPodsModel): ModelDetails {
  return AirPodsModelDetails[model] || AirPodsModelDetails["Unknown"];
}
