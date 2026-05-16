# Signal in the Silverstream - Puzzle Graph

## Nodes

| ID | Beat | Technique/device | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|---|
| P1 | Route fragments | `TECH-SORT-002`, `TECH-ALIGN-001` / `DEV-TRAY-001` | observation, ordering, association | postcards, wall map, mileage plaques | postcards, map, mileage plaques | postcards are stops, not souvenirs | ordered route code | shuffle postcards |
| P2 | Shore-power panel | `TECH-SORT-002`, `TECH-REVEAL-002` / `DEV-SWITCH-001` | indexing, ordering, tactile reasoning | prop breaker panel with safe low-voltage switches | route code, breaker labels | power labels map to route stops | safe power sequence | reset switches |
| P3 | Galley inventory | `TECH-SORT-004`, `TECH-REVEAL-002` / `DEV-LED-001` | observation, extraction, classification | UV-lit cabinet and removable galley objects | powered UV strip, cabinet items | missing objects mark radio frequencies | three frequency digits | replace objects |
| P4 | Fold-down table | `TECH-ALIGN-004`, `TECH-REVEAL-003` / `DEV-FOLD-001` | spatial reasoning, transformation | rotating/latching table with etched compass rose and service-notch bearing mark | frequencies, etched compass rose, service notch | table orientation changes the dial meaning | bearing word | reset table latch |
| P5 | Radio convergence | `TECH-TEAM-002`, `TECH-REVEAL-004` / `DEV-DIAL-001` | metapuzzle, audio reasoning, collaboration | tactile radio dial, route display, and final audio playback | route, power, frequency, bearing | all systems tune one impossible trip | final broadcast and exit key | reset radio state |

## Technique play profile

| Beat/candidate | Technique(s) | Crowd profile | Advantage in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 Route fragments | `TECH-SORT-002`, `TECH-ALIGN-001` | helper | makes travel route logic physical and public | postcards become private reading/search soup | map silhouettes, mileage plaques, low card count |
| P2 Shore-power panel | `TECH-SORT-002`, `TECH-REVEAL-002` | tactile helper | safe switching makes the trailer system feel real | random toggling or light-state ambiguity | fused low voltage, visible/manual accepted state |
| P3 Galley inventory | `TECH-SORT-004`, `TECH-REVEAL-002` | discovery helper | missing objects and powered reveal connect inventory to radio | UV-only clue or object reset debt | visible backup labels, object count, duplicate set |
| P4 Fold-down table | `TECH-ALIGN-004`, `TECH-REVEAL-003` | transformation pleaser | trailer furniture changes meaning in a memorable way | latch force, reach, or players leaning on table | no-load stop, staff compass fallback, reset photo |
| P5 Radio convergence | `TECH-TEAM-002`, `TECH-REVEAL-004` | finale pleaser with audio risk | tactile tuning plus broadcast creates the impossible-trip payoff | fine tuning or audio-only success | detents/big marks, transcript/visual confirmation |

## Edges

| From | To | Dependency | Failure fallback |
|---|---|---|---|
| P1 | P2 | route order feeds breaker sequence | operator hint: "The stops are in driving order." |
| P2 | P3 | UV strip powers after correct sequence | visible backup labels after soft warning |
| P3 | P4 | frequency digits unlock table clue | operator can reveal one missing object or one frequency digit |
| P4 | P5 | bearing word disambiguates final tuning | service-notch bearing mark, then final acceleration card at hard-exit minus 3 |

## Parallelism

P1 and parts of P3 can be explored simultaneously. P2 is the intentional early
bottleneck because it teaches the room's "trailer systems matter" contract.
P5 is the final convergence.

## Open design risks

- Avoid making P1 pure search.
- Ensure P2 is safe: breaker props must not expose real electrical risk.
- Validate that the P4 service-notch bearing mark accelerates stuck teams without making the aha feel automatic.
- Keep P5 operator-accelerable without feeling like a bailout.
