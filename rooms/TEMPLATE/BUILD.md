# Build Template

Use `docs/build-economics.md`, `components/TECHNIQUES.md`,
`components/DEVICES.md`, `components/KITS.md`,
`components/MATERIALS.md`, `components/DEVICE-REVIEW.md`,
`components/INVENTORY.md`, and `components/SOURCING.md` before build readiness
review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | TBD | |
| Production | TBD | |
| Spares | TBD | |
| Maintenance | TBD | |
| Transport | TBD | |
| Build hours | TBD | total maker/fabrication/staging time |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|

## Part diagrams

Use `components\VISUALS.md`.

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|

## Bill of materials

| Inventory ID | Component | Beat IDs | Visual reference | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|

## Materials and construction plan

| Beat/device | Prototype material/recipe | Public-use material | Maker fit | Child/teen helper task | Adult-only step | Finish/transport note | Trigger to upgrade |
|---|---|---|---|---|---|---|---|

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|

## Criticality map

Use `components\RELIABILITY.md`.

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | TBD | TBD | spare plus bypass/manual release | timed drill |
| C4 required | TBD | TBD | duplicate or ready spare | timed drill |
| C3 beat-support | TBD | TBD | spare or printed backup | reset/replacement check |
| C2 helper | TBD | TBD | optional spare | post-run repair |
| C1 cosmetic | TBD | TBD | repair consumable | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | TBD | TBD | TBD |
| BOM line for every required component | TBD | TBD | TBD |
| Inventory ID or custom fabrication plan for every critical component | TBD | TBD | TBD |
| Material and construction plan for every critical component | TBD | TBD | TBD |
| Technique/device/kit IDs for every required beat | TBD | TBD | TBD |
| Visual reference for every C4-C5 component | TBD | TBD | TBD |
| Criticality, spare count, build time, and replacement time assigned | TBD | TBD | TBD |
| C4-C5 admin replacement drill passes in time | TBD | TBD | TBD |
| Device review level/risk band assigned | TBD | TBD | TBD |
| Known cost band for critical components | TBD | TBD | TBD |
| Manual bypass for powered/sensed/fragile beats | TBD | TBD | TBD |
| Durability class not D for required components | TBD | TBD | TBD |
| Reset action verifiable under time pressure | TBD | TBD | TBD |
