# The Apple Continuity proximity pairing payload

Where every battery percentage in winpods comes from.

Apple devices broadcast unauthenticated BLE manufacturer-specific data under Bluetooth SIG company
id **76** (`0x004C`). One of those message types — *proximity pairing*, type `0x07` — carries the
battery levels, charging flags, lid state and in-ear detection for AirPods-style devices. Reading it
requires no pairing, no authentication and no connection: any nearby listener can decode it.

**None of this is documented by Apple.** The layout below is community reverse engineering. Every
accessor is covered by unit tests in `crates/apple-cp/src/proximity_pairing.rs` so a regression
shows up as a failing test rather than a wrong percentage on screen.

## Layout

27 bytes, after the company id has been stripped (which is what
`BluetoothLEManufacturerData::Data` yields).

| Offset | Size | Field | Notes |
| --- | --- | --- | --- |
| 0 | 1 | packet type | Must be `0x07` |
| 1 | 1 | remaining length | Must be `25` (`27 - 2`) |
| 2 | 1 | *unknown* | Not decoded |
| 3 | 2 | model id | **Little endian**: AirPods Pro `0x200E` arrives as `0E 20` |
| 5 | 1 | status flags | See below |
| 6 | 2 | battery | See below |
| 8 | 1 | lid state | Bit `0x08` set = lid **closed** |
| 9 | 1 | colour | Apple's table is incomplete; exposed as a raw byte |
| 10 | 1 | *unknown* | Not decoded |
| 11 | 16 | hash / encrypted payload | **Deliberately not stored** |

The trailing 16 bytes are opaque and potentially identifying, so `ProximityPairingMessage` does not
retain them. That is what makes a decoded message safe to log, and there is a test asserting two
payloads differing only there decode identically.

### Status flags (offset 5)

| Bit | Meaning |
| --- | --- |
| `0x02` | The broadcasting bud is in an ear |
| `0x04` | Both buds are in the case |
| `0x08` | The *other* bud is in an ear |
| `0x20` | Set = broadcast from the **left** bud, clear = from the **right** |

### Battery (offsets 6–7)

Byte 6 packs two levels as nibbles:

| Nibble | Meaning |
| --- | --- |
| low (`& 0x0F`) | The **broadcasting** bud |
| high (`>> 4`) | The **other** bud |

Byte 7:

| Bits | Meaning |
| --- | --- |
| `& 0x0F` | Case level |
| `0x10` | The broadcasting bud is charging |
| `0x20` | The other bud is charging |
| `0x40` | The case is charging |

## Decoding rules that are easy to get wrong

**Levels are tenths, and >10 means unknown.** A nibble of `0..=10` maps to `0..=100` percent.
Anything above 10 (commonly `0x0F`) means *no reading*, which happens routinely for a bud sitting in
a closed case. This is why every battery is an `Option` throughout the codebase: a bud with no
reading is not a bud at 0%.

**"Current" and "other" are relative to the sender.** Buds alternate who broadcasts, so the same
physical bud is sometimes "current" and sometimes "other". Resolving left/right requires the
broadcast-side flag (`0x20`); ignoring it makes the two batteries swap at random.

**Charging beats the in-ear bit.** A charging bud is in the case by definition, but the in-ear bit
is not cleared reliably. `is_left_in_ear` / `is_right_in_ear` therefore return `false` whenever that
side reports charging.

**Over-ear models still populate the case nibble.** AirPods Max have no case, so
`DeviceProperties::from_advertisement` drops the case reading for those models via
`AppleDeviceModel::has_case`. Otherwise the UI draws a case that does not exist.

## Model ids

Mapped in `crates/apple-cp/src/model.rs`.

| Id | Model | Id | Model |
| --- | --- | --- | --- |
| `0x2002` | AirPods | `0x200A` | AirPods Max |
| `0x200F` | AirPods (2nd gen) | `0x201F` | AirPods Max (USB-C) |
| `0x2013` | AirPods (3rd gen) | `0x200B` | Powerbeats Pro |
| `0x2019` | AirPods 4 | `0x201D` | Powerbeats Pro 2 |
| `0x201B` | AirPods 4 (ANC) | `0x2012` | Beats Fit Pro |
| `0x200E` | AirPods Pro | `0x2011` | Beats Studio Buds |
| `0x2014` | AirPods Pro 2 | `0x2016` | Beats Studio Buds + |
| `0x2024` | AirPods Pro 2 (USB-C) | `0x2026` | Beats Solo Buds |
| `0x2027` | AirPods Pro 3 | | |

Unknown ids fall back to `AppleDeviceModel::Unknown` and still render, just without correct artwork
or a name. Adding a model means adding the id here and an entry in
`crates/desktop/src/lib/models.ts`.

## Distinguishing devices

Advertisements carry no stable identifier — the address is randomised and the payload is identical
across two of the same model. When several candidates are in range, winpods relies on:

1. The **model** must match the selected device's product id (read from the paired Windows device,
   not from the advertisement).
2. The readings must be **plausible** relative to the last accepted ones: within 50 dBm of signal
   strength and 20 percentage points of battery
   (`DeviceProperties::is_plausible_update`).

This is heuristic. Two identical pairs at a similar distance and charge level cannot be told apart
from advertisements alone.
