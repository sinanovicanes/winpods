import airpodBud from "$lib/assets/airpod-b.png";
import airpodProBud from "$lib/assets/airpod-pro-b.png";
import airpodsCase from "$lib/assets/airpods-case.png";
import airpodsProCase from "$lib/assets/airpods-pro-case.png";
import airpodsHero from "$lib/assets/airpods.webp";
import airpodsProHero from "$lib/assets/airpods_pro_2.webp";
import airpodsMaxBud from "$lib/assets/airpods_max.png";
import airpodsMaxHero from "$lib/assets/airpods-max.png";

import type { AppleDeviceModel } from "$lib/ipc";

export interface ModelArtwork {
  /** Display name shown in the dashboard. */
  name: string;
  /** Large product shot for the dashboard. */
  hero: string;
  /** Small image for the widget. */
  bud: string;
  /** Case image, absent for over-ear models that have no case. */
  case?: string;
  /** Whether the bud image should be mirrored to make a pair. */
  pair: boolean;
}

const inEar = (
  name: string,
  hero: string,
  bud: string,
  caseImage: string
): ModelArtwork => ({
  name,
  hero,
  bud,
  case: caseImage,
  pair: true
});

const overEar = (name: string): ModelArtwork => ({
  name,
  hero: airpodsMaxHero,
  bud: airpodsMaxBud,
  pair: false
});

/**
 * Artwork and display names per model.
 *
 * Keyed by the exact strings `winpods_apple_cp::AppleDeviceModel` serialises to.
 *
 * NOTE: the Beats models reuse the AirPods artwork as a placeholder -- correct product shots for
 * them are still missing, which is a known gap carried over from v0.1.
 */
export const MODEL_ARTWORK: Record<AppleDeviceModel, ModelArtwork> = {
  AirPods1: inEar("AirPods", airpodsHero, airpodBud, airpodsCase),
  AirPods2: inEar("AirPods (2nd gen)", airpodsHero, airpodBud, airpodsCase),
  AirPods3: inEar("AirPods (3rd gen)", airpodsHero, airpodBud, airpodsCase),
  AirPods4: inEar("AirPods 4", airpodsHero, airpodBud, airpodsCase),
  AirPods4Anc: inEar("AirPods 4 (ANC)", airpodsHero, airpodBud, airpodsCase),
  AirPodsPro: inEar("AirPods Pro", airpodsProHero, airpodProBud, airpodsProCase),
  AirPodsPro2: inEar("AirPods Pro 2", airpodsProHero, airpodProBud, airpodsProCase),
  AirPodsPro2UsbC: inEar(
    "AirPods Pro 2 (USB-C)",
    airpodsProHero,
    airpodProBud,
    airpodsProCase
  ),
  AirPodsPro3: inEar("AirPods Pro 3", airpodsProHero, airpodProBud, airpodsProCase),
  AirPodsMax: overEar("AirPods Max"),
  AirPodsMaxUsbC: overEar("AirPods Max (USB-C)"),
  PowerbeatsPro: inEar("Powerbeats Pro", airpodsProHero, airpodProBud, airpodsProCase),
  PowerbeatsPro2: inEar("Powerbeats Pro 2", airpodsProHero, airpodProBud, airpodsProCase),
  BeatsFitPro: inEar("Beats Fit Pro", airpodsProHero, airpodProBud, airpodsProCase),
  BeatsStudioBuds: inEar(
    "Beats Studio Buds",
    airpodsProHero,
    airpodProBud,
    airpodsProCase
  ),
  BeatsStudioBudsPlus: inEar(
    "Beats Studio Buds +",
    airpodsProHero,
    airpodProBud,
    airpodsProCase
  ),
  BeatsSoloBuds: inEar("Beats Solo Buds", airpodsProHero, airpodProBud, airpodsProCase),
  Unknown: inEar("Unknown device", airpodsHero, airpodBud, airpodsCase)
};

export function artworkFor(model: AppleDeviceModel | undefined): ModelArtwork {
  return (model && MODEL_ARTWORK[model]) || MODEL_ARTWORK.Unknown;
}
