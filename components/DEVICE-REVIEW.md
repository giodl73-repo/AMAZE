# AMAZE Device Review Matrix

Use this matrix before a device becomes part of a build-candidate room. It keeps
cool props from outrunning safety, reset, operator control, and public-use
reliability.

Use `components/RELIABILITY.md` for failure IDs, chaos probes, FMEA-lite fields,
and bench result states.

## Review levels

| Level | Meaning | Evidence required |
|---|---|---|
| L0 idea | Device is a sketch or inspiration. | Aha, player action, and likely technique named. |
| L1 cardboard | Device has a low-cost mockup. | Wrong attempts, reset, and accessibility checked with staff observing. |
| L2 bench prototype | Device uses intended mechanism class. | Repeatability, abuse case, bypass, and spare plan tested. |
| L3 room prototype | Device works inside room flow. | Timing, operator visibility, reset, and team behavior logged. |
| L4 public-ready | Device can survive schedule pressure. | Maintenance interval, spare kit, failure handling, and safety review passed. |

## Required review table

| Review field | Required answer |
|---|---|
| Technique IDs | Which `TECH-*` patterns does this device implement? |
| Device ID | Existing `DEV-*` ID or new proposed ID. |
| Kit IDs | Prototype, ops, electronics, mechanical, transport, or maintenance kits used. |
| Inventory IDs | Repeatable parts and acceptable substitutes. |
| Visual reference | Which `components\diagrams\*.excalidraw` or room-specific diagram shows player face, build, reset, failure, and recovery? |
| Player success signal | What do players see/feel/hear when it works? |
| Operator success signal | What can staff see without entering the room? |
| Crowd profile | Is this expected to be a crowd pleaser, helper, neutral support, or frustration risk? |
| Frustration trigger | What specific player behavior or device state makes it stop being fun? |
| False positive | How could it say yes when players are wrong? |
| False negative | How could it say no when players are right? |
| Abuse case | Pull, twist, drop, pocket, spill, force, climb, or block scenario. |
| Reliability issue IDs | Which `REL-*` failures are likely? |
| Chaos probes | Which `CHAOS-*` probes must be run? |
| Accessibility path | How a seated, low-vision, low-hearing, anxious, or mobility-limited player can participate. |
| Manual bypass | Staff action that preserves safety and pace. |
| Reset check | Fast, visible check between teams. |
| Spare plan | What duplicate part exists, and where it lives. |
| Kill criteria | What result forces redesign instead of patching. |

## Device risk bands

| Band | Typical device | Required proof |
|---|---|---|
| Green | printed cards, bins, oversized tokens, passive trays | count/reset proof and duplicate print set |
| Yellow | rails, sockets, overlays, route boards, hinged covers | wrong-attempt test, no-force behavior, reset photo |
| Orange | sensors, lights, motors/fans, microcontrollers, audio | manual equivalent, power/fuse review, false-state test |
| Red | heat, smoke, mist/water, darkness reliance, high reach, crawl, load-bearing structure | avoid by default; safety governance review before prototype |

## Crowd profile bands

| Band | Meaning | Promote when | Revise when |
|---|---|---|---|
| Pleaser | Players are likely to remember and describe the device. | Success is public, tactile, fair, and thematic. | The effect is bigger than the aha or masks confusion. |
| Helper | Device improves flow, reset, or understanding without being the star. | It reduces search, supports roles, or clarifies progress. | It feels like admin, sorting, or staff bookkeeping. |
| Neutral support | Device is necessary infrastructure. | It is reliable, quiet, and does not attract false attention. | Players mistake it for an unsolved clue. |
| Frustration risk | Device can be fun but often fails through ambiguity, precision, or reliability. | Bench tests show wrong attempts fail clearly and safely. | Players blame hardware instead of revising their idea. |

## Kill criteria

Redesign the device instead of patching it if any condition appears twice:

1. Players can force a false solve.
2. Staff cannot see whether it is solved.
3. Reset takes longer than the beat's play time.
4. Success depends on hearing, darkness, color alone, or fine motor precision.
5. The device creates pinch, trip, climb, wet/slip, heat, or electrical risk.
6. The room cannot continue safely when the device fails.
7. Replacement parts are one-off and no substitute exists.

## Bench test script

Run each required device through this script before room prototype:

1. Correct-use pass: one player solves it normally and explains the aha.
2. Wrong-use pass: one player tries plausible wrong objects/states.
3. Chaotic-use pass: one player rushes, dumps, twists, or guesses.
4. Accessibility pass: staff assign a seated checker or visual/audio alternative.
5. Operator pass: staff observe, hint, bypass, and reset without designer help.
6. Transport pass: device is packed, moved, unpacked, and checked against reset photo.

Record results in the room `PLAYTEST.md`, then update the room `BUILD.md` device
level and risk band.

## Visual review gate

For any C4 required or C5 showstopper device, the review is incomplete until a
diagram shows:

| Visual check | Why it matters |
|---|---|
| Player face | Confirms the action is physical and legible, not just a hidden logic state. |
| Hidden build | Exposes stops, hinges, sockets, rails, sensors, fasteners, and pinch/force risks. |
| Reset state | Shows what staff must verify from a photo, count, camera, or checklist. |
| Failure path | Shows how loss, jam, swap, false fit, glare, or force happens. |
| Admin recovery | Shows where the spare/bypass lives and the target replacement time. |
| Simulator fields | Connects the drawing to `Criticality`, `Build time`, `Replacement min`, `Spare qty`, and `Failure mode`. |
