# Build Template

Use `docs/build-economics.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | TBD | |
| Production | TBD | |
| Spares | TBD | |
| Maintenance | TBD | |
| Transport | TBD | |

## Puzzle mechanism map

| Beat ID | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|

## Spare kit

| Spare | Quantity | Stored where | Replace when |
|---|---:|---|---|

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | TBD | TBD | TBD |
| BOM line for every required component | TBD | TBD | TBD |
| Inventory ID or custom fabrication plan for every critical component | TBD | TBD | TBD |
| Known cost band for critical components | TBD | TBD | TBD |
| Manual bypass for powered/sensed/fragile beats | TBD | TBD | TBD |
| Durability class not D for required components | TBD | TBD | TBD |
| Reset action verifiable under time pressure | TBD | TBD | TBD |
