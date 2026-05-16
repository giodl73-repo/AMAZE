# Signal in the Silverstream - Build Economics

This is an initial build pass, not a vendor quote.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | TBD | paper route set, mock switch panel, temporary UV cabinet, radio audio test |
| Production | TBD | mounted panels, durable table mechanism, finished scenic props |
| Spares | TBD | duplicate postcards, labels, cabinet objects, switch caps, radio knobs |
| Maintenance | TBD | batteries, cleaning, replacement labels, calibration checklist |
| Transport | TBD | padded cases for removable props and shock protection for electronics |

## Puzzle mechanism map

| Beat ID | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|
| P1 | postcards, wall map, mileage plaques | analog | operator confirms next stop | shuffle postcards | cards bent, pocketed, or marked |
| P2 | low-voltage prop breaker switches | electrical | operator triggers UV circuit manually | reset switches | random force on switches |
| P3 | UV-lit cabinet and removable galley objects | lighting | reveal one frequency digit | replace objects | objects dropped, hidden, or pocketed |
| P4 | rotating/latching table with etched compass rose | analog/mechanical | operator points to compass mark | reset latch and orientation | players lean on or force table |
| P5 | tactile radio dial and audio playback | audio/electrical | operator plays final broadcast | reset station and audio state | knob loosened or audio misfires |

## Bill of materials

| Component | Beat IDs | Quantity | Unit cost | Source | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---:|---:|---|---|---|---|---|---|
| laminated postcards | P1 | 2 sets | TBD | print/fabricate | TBD | B | bending, loss, markings | spare set | wipe and inspect |
| mounted route map | P1 | 1 | TBD | print/fabricate | TBD | A | scratches or peeling | operator verbal clue | clean, reattach |
| prop switch panel | P2 | 1 plus spare caps | TBD | fabricate | TBD | Unknown | switch false negative | manual UV trigger | test before session |
| UV cabinet strip | P3 | 1 plus spare | TBD | vendor/fabricate | TBD | B | light failure | reveal backup labels | test, replace strip |
| galley object set | P3 | 2 sets | TBD | source/fabricate | TBD | C | missing object | spare set | count after reset |
| rotating table latch | P4 | 1 plus spare latch | TBD | fabricate | TBD | Unknown | jams or loosens | operator compass clue | tighten and test |
| radio dial and audio module | P5 | 1 plus spare knob | TBD | vendor/fabricate | TBD | Unknown | bad read or audio failure | operator playback | pre-run audio test |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| switch panel fails to trigger UV | P2, P3 | operator sees correct sequence but no light | manual UV trigger | separate puzzle logic from fragile circuit |
| table latch jams | P4 | players find compass but cannot rotate/latch | point to backup bearing clue | prototype latch under force |
| radio audio misfires | P5 | correct tuning with no playback | operator plays final broadcast | add visible success indicator |

## Spare kit

| Spare | Quantity | Stored where | Replace when |
|---|---:|---|---|
| postcard set | 1 | operator kit | any card is damaged or missing |
| galley object set | 1 | reset bin | object missing or visibly damaged |
| switch caps | 3 | parts pouch | cap cracks or label wears |
| UV strip | 1 | parts pouch | strip flickers or fails pre-run |
| radio knob | 1 | parts pouch | knob loosens or slips |
