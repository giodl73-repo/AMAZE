# Build Economics and Reliability

An AMAZE puzzle is not real until it has a physical mechanism and a build
economics record. Theme, story, and codes can motivate the beat, but the player
must do something embodied: touch, move, align, listen, weigh, tune, assemble,
route, reveal, or coordinate.

## Physicality rule

Every promoted puzzle beat must declare:

| Field | Question |
|---|---|
| Physical mechanism | What do players physically manipulate, observe, hear, align, or trigger? |
| Device class | None, analog, lock, magnet, electrical, sensor, lighting, audio, screen, networked. |
| Manual fallback | How can staff keep play moving if the device fails? |
| Reset state | What must return to start state between teams? |
| Abuse case | What happens if players pull, twist, drop, pocket, spill, or force it? |

Pure paper/code puzzles can exist only when physically grounded in the set and
paired with a tactile or spatial action.

## BOM line item

Use `components/INVENTORY.md` first. Room BOMs should reference an inventory ID
when possible, then record any room-specific quantity, source, and cost decision.
If an item is custom or one-off, mark it as `CUSTOM-*` and add a substitute or
fabrication plan before build readiness review.

Use this for every component:

| Field | Meaning |
|---|---|
| Inventory ID | Reusable component ID from `components/INVENTORY.md`, or `CUSTOM-*`. |
| Component | Named part or assembly. |
| Beat IDs | Puzzle beats that depend on it. |
| Installed qty | Count installed in the room. |
| Spare qty | Minimum spare count for public operation. |
| Unit cost band | Current estimate or verified cost band. |
| Source tier | Common supplier class or fabrication path. |
| Price confidence | Planning, checked, quoted, or purchased. |
| Lead time | Known or estimated procurement delay. |
| Durability class | Low, medium, high, consumable, unknown. |
| Failure mode | How it breaks or misreads. |
| Bypass | Operator action when it fails mid-session. |
| Maintenance | Cleaning, charging, calibration, tightening, replacement. |

## Budget bands

Track budgets separately:

| Budget | Includes |
|---|---|
| Prototype | rough parts, temporary fixtures, test electronics |
| Production | final materials, scenic finish, mounted hardware |
| Spares | replacements for high-touch and fragile parts |
| Maintenance | consumables, batteries, cleaning, calibration tools |
| Transport | cases, vibration protection, tie-downs, weather covers |

## Common sourcing

Prefer repeatable sources:

1. Big-box hardware for wood, fasteners, hinges, brackets, cable raceway, and
   basic lighting.
2. Electronics suppliers for switches, low-voltage power, LEDs, sensors, and
   connectors.
3. Industrial suppliers for robust latches, magnets, hinges, labels, cases, and
   serviceable hardware.
4. Office/print shops for clue cards, laminated evidence, labels, and reset
   sheets.
5. Thrift/scenic sources only for non-critical dressing unless the item has a
   documented substitute.

See `components/SOURCING.md` for supplier tiers and price confidence rules.

## Durability review

Score each component:

| Rating | Meaning |
|---|---|
| A | expected to survive repeated public use with routine cleaning |
| B | durable with inspection or occasional tightening/replacement |
| C | likely to wear; needs spares and operator checks |
| D | fragile, fiddly, or unsafe for public use; redesign before build |
| Unknown | must prototype before promotion |

## Device reliability review

Any powered or sensed beat must answer:

1. What does success look like to the player?
2. What does success look like to the operator?
3. What does failure look like to the player?
4. Can the operator detect false negatives and false positives?
5. Is there a manual bypass that preserves the fiction?
6. Does the room still exit safely if the device dies?

## Build readiness gate

A room fails build readiness if any required beat has:

- no physical mechanism;
- no BOM line item;
- no inventory ID or custom fabrication plan for a critical component;
- unknown cost band for a critical component;
- no bypass for an electronic or sensor device;
- a durability rating of D;
- a reset action that staff cannot verify under time pressure.
