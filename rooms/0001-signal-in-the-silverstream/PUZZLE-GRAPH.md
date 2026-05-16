# Signal in the Silverstream - Puzzle Graph

## Nodes

| ID | Beat | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|
| P1 | Route fragments | observation, ordering, association | postcards, wall map, mileage plaques | postcards, map, mileage plaques | postcards are stops, not souvenirs | ordered route code | shuffle postcards |
| P2 | Shore-power panel | indexing, ordering, tactile reasoning | prop breaker panel with safe low-voltage switches | route code, breaker labels | power labels map to route stops | safe power sequence | reset switches |
| P3 | Galley inventory | observation, extraction, classification | UV-lit cabinet and removable galley objects | powered UV strip, cabinet items | missing objects mark radio frequencies | three frequency digits | replace objects |
| P4 | Fold-down table | spatial reasoning, transformation | rotating/latching table with etched compass rose and service-notch bearing mark | frequencies, etched compass rose, service notch | table orientation changes the dial meaning | bearing word | reset table latch |
| P5 | Radio convergence | metapuzzle, audio reasoning, collaboration | tactile radio dial, route display, and final audio playback | route, power, frequency, bearing | all systems tune one impossible trip | final broadcast and exit key | reset radio state |

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
