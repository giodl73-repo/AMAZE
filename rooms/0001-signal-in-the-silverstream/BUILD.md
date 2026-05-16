# Signal in the Silverstream - Build Economics

This is an initial build pass, not a vendor quote.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$200-600` | paper route set, mock switch panel, temporary UV cabinet, radio audio test |
| Production | `$600-2,000+` | mounted panels, durable table mechanism, finished scenic props |
| Spares | `$75-250` | duplicate postcards, labels, cabinet objects, switch caps, radio knobs |
| Maintenance | `$25-100/run season` | batteries, cleaning, replacement labels, calibration checklist |
| Transport | `$50-200` | padded cases for removable props and shock protection for electronics |

## Puzzle mechanism map

| Beat ID | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|
| P1 | postcards, wall map, mileage plaques | analog | operator confirms next stop | shuffle postcards | cards bent, pocketed, or marked |
| P2 | low-voltage prop breaker switches | electrical | operator triggers UV circuit manually | reset switches | random force on switches |
| P3 | UV-lit cabinet and removable galley objects | lighting | reveal one frequency digit | replace objects | objects dropped, hidden, or pocketed |
| P4 | rotating/latching table with etched compass rose | analog/mechanical | operator points to compass mark | reset latch and orientation | players lean on or force table |
| P5 | tactile radio dial and audio playback | audio/electrical | operator plays final broadcast | reset station and audio state | knob loosened or audio misfires |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| PRINT-001 | laminated postcards | P1 | 1 set | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bending, loss, markings | spare set | wipe and inspect |
| PRINT-001 or CUSTOM-PRINT | mounted route map | P1 | 1 | 1 reprint file | `$5-60` | print shop or local fabrication | Planning | 0-1 week | B | scratches or peeling | operator verbal clue | clean, reattach |
| ELEC-002 | prop breaker switches | P2 | 4-8 | 2 | `$2-12` each | Digi-Key, Mouser, Adafruit, Amazon | Planning | 0-1 week | B | switch false negative | manual UV trigger | test before session |
| ELEC-001 | UV cabinet strip | P3 | 1 | 1 strip | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light failure | reveal backup labels | test, replace strip |
| SCENIC-005 or CUSTOM-SCENIC | galley object set | P3 | 1 set | 1 set | `$15-75` | general retail, thrift/scenic, local fabrication | Planning | 0-2 weeks | C | missing object | spare set | count after reset |
| MECH-001 or MECH-004 | rotating table latch/track | P4 | 1 | 1 latch | `$12-70` | big-box hardware, McMaster-Carr, local fabrication | Planning | 0-2 weeks | Unknown | jams or loosens | operator compass clue | tighten and test |
| MECH-006 | radio dial/knob | P5 | 1 | 1 knob | `$3-25` | Amazon, McMaster-Carr, electronics suppliers | Planning | 0-1 week | B | knob loosened or slips | operator playback | inspect set screw |
| ELEC-006 | audio module/player | P5 | 1 | 1 optional | `$15-50` | Amazon, Adafruit audio board, thrift retail | Planning | 0-1 week | C | audio failure | operator playback | pre-run audio test |

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
