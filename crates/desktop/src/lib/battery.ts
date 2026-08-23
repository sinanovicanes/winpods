import type { Battery } from "$lib/ipc";

export type BatteryTone = "good" | "low" | "critical" | "unknown";

/**
 * Classifies a battery reading for colouring.
 *
 * A charging battery is always `good`: the level is on its way up, so colouring it red is noise.
 * `null` means the device reported nothing — a bud in a closed case — which is distinct from 0%.
 */
export function batteryTone(level: number | null, charging = false): BatteryTone {
  if (level === null) return "unknown";
  if (charging) return "good";
  if (level <= 10) return "critical";
  if (level <= 20) return "low";
  return "good";
}

/** Fill colour for a bar or ring. */
export const TONE_FILL: Record<BatteryTone, string> = {
  good: "bg-battery-good",
  low: "bg-battery-low",
  critical: "bg-battery-critical",
  unknown: "bg-transparent"
};

/** Text colour on a light surface. Healthy levels stay neutral so only problems draw the eye. */
export const TONE_TEXT: Record<BatteryTone, string> = {
  good: "text-foreground",
  low: "text-battery-low",
  critical: "text-battery-critical",
  unknown: "text-muted-foreground"
};

/** Text colour on the widget's dark translucent panel. */
export const TONE_TEXT_ON_DARK: Record<BatteryTone, string> = {
  good: "text-white",
  low: "text-battery-low",
  critical: "text-battery-critical",
  unknown: "text-white/40"
};

/** A battery reading paired with the side it belongs to, as the UI renders it. */
export interface BatteryReading {
  label: string;
  /** Short form for tight spaces, e.g. the widget. */
  short: string;
  battery: Battery | null;
  inEar?: boolean;
}
