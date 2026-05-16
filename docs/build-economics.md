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
| Technique | Which reusable pattern from `components/TECHNIQUES.md` is this beat using? |
| Beat package | Which reusable beat package from `components/BEAT-PACKAGES.md` defines the expected room data, evidence hooks, and promotion criteria? |
| Device/module | Which reusable module from `components/DEVICES.md` or custom device implements it? |
| Kit | Which reusable bundle from `components/KITS.md` supports prototype, operation, electronics, mechanics, transport, or maintenance? |
| Materials | Which `MAT-*` materials and `MAKE-*` construction recipe from `components/MATERIALS.md` prove and then productionize it? |
| Maker fit | Can a child or teen help safely, and which steps are adult-only? |
| Crowd profile | Is the technique/device expected to be a pleaser, helper, neutral support, or frustration risk? |
| Frustration trigger | What makes this stop being fun for players? |
| Reliability issues | Which `REL-*` failures from `components/RELIABILITY.md` are likely? |
| Chaos probes | Which `CHAOS-*` tests must this device survive? |
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

Use `components/TECHNIQUES.md`, `components/BEAT-PACKAGES.md`,
`components/DEVICES.md`, `components/KITS.md`, `components/MATERIALS.md`,
`components/DEVICE-REVIEW.md`, and `components/RELIABILITY.md` before inventing
a new mechanism. New device ideas are welcome, but they should be added to the
catalog once a second room could reuse them.

Use this for every component:

| Field | Meaning |
|---|---|
| Inventory ID | Reusable component ID from `components/INVENTORY.md`, or `CUSTOM-*`. |
| Component | Named part or assembly. |
| Beat IDs | Puzzle beats that depend on it. |
| Visual reference | Excalidraw part card or room-specific diagram for C4-C5 components. |
| Installed qty | Count installed in the room. |
| Spare qty | Minimum spare count for public operation. |
| Criticality | C1 cosmetic, C2 helper, C3 beat-support, C4 required, or C5 showstopper. |
| Build time | Estimated maker/fabrication hours for the installed item and ready spares. |
| Replacement min | Minutes staff need to replace, hand in, or bypass the item during a run. |
| Unit cost band | Current estimate or verified cost band. |
| Source tier | Common supplier class or fabrication path. |
| Price confidence | Planning, checked, quoted, or purchased. |
| Lead time | Known or estimated procurement delay. |
| Durability class | Low, medium, high, consumable, unknown. |
| Failure mode | How it breaks or misreads. |
| Bypass | Operator action when it fails mid-session. |
| Admin recovery | Where the spare/bypass is staged and how staff deploy it under pressure. |
| Maintenance | Cleaning, charging, calibration, tightening, replacement. |

## Materials and construction

Room build packs should separate:

| Layer | Question |
|---|---|
| Prototype material | What cheap/found/craft material proves the aha? |
| Public-use material | What durable, cleanable, resettable material replaces it? |
| Construction recipe | Which `MAKE-*` recipe or custom method builds it? |
| Maker fit | Which steps are child helper, teen helper, adult supervised, or adult only? |
| Finish | How is it sealed, labeled, rounded, cleaned, and transported? |

Craft builds are encouraged for prototypes. Public-use builds must upgrade any
fragile, messy, sharp, wet, food, glitter, glass, loose-magnet, or load-bearing
material before playtest-ready promotion.

## Build time, replaceability, and backups

Every room build pack should answer these before fabrication:

| Question | Required answer |
|---|---|
| How long does the room take to build? | Prototype hours, production hours, adult-only hours, and curing/sealing wait time. |
| How long does each item take to replace? | `Replacement min` in the BOM, proven by an admin replacement drill for C3-C5 items. |
| How many backups does each item need? | Based on `components\RELIABILITY.md` criticality: C1-C2 may be repair-only; C3 needs a spare or printed backup; C4 needs a ready duplicate; C5 needs two recovery paths. |
| Can staff recover in time? | Yes only if the spare/bypass is labeled, reachable, and usable within beat slow-case slack. |
| What if the item is not found? | Count card, tether, duplicate, or operator hand-in must keep the solve fair. |

Do not count a spare as real unless staff can find it under pressure. A duplicate
buried in a transport tote is inventory, not live recovery.

## Visual build review

Use `components\VISUALS.md` with the BOM. For C4 required and C5 showstopper
items, a text row is not enough before build readiness. The item needs a visual
reference that shows:

| Visual layer | Required evidence |
|---|---|
| Player face | The exact thing players touch, move, align, inspect, or place. |
| Hidden build | Stops, hinges, rails, sockets, fasteners, sensors, or support structure. |
| Reset state | Count/photo/camera-visible state staff can verify. |
| Failure path | What breaks, jams, is lost, swapped, forced, or misread. |
| Admin recovery | Spare or bypass location plus replacement target. |
| Upgrade path | Prototype material and public-use material. |

The harness enforces this in `amaze check`: a C4-C5 BOM row must either include a
`Visual reference`/`Part diagram`/`Diagram` field pointing to an existing
`.excalidraw` file, or be covered by a matching row in `BUILD.md` -> `## Part
diagrams`.

The same row must declare what the diagram proves and what is still missing
before build readiness. A generic reusable card is acceptable for a draft pass
only if the remaining room-specific gap is explicit, such as final dimensions,
sightline, hinge stop, no-force proof, pouch map, or reset photo.

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

Use `components/DEVICE-REVIEW.md` for any yellow, orange, or red risk-band
device before build-candidate promotion.

## Build readiness gate

A room fails build readiness if any required beat has:

- no physical mechanism;
- no BOM line item;
- no inventory ID or custom fabrication plan for a critical component;
- no criticality, spare rule, build time, or replacement time for C3-C5 components;
- unknown cost band for a critical component;
- no bypass for an electronic or sensor device;
- a durability rating of D;
- a reset action that staff cannot verify under time pressure.
