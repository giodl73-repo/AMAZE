# Puzzle-Type Phases

AMAZE uses puzzle types as design phases. A phase is a focused learning track:
we choose a mechanic family, build a small wave of candidates, pulse-test them
through simulation, and promote only the beats that become physical, fair,
resettable, safe, and thematic inside a trailer.

Waves and pulses are **AMAZE design cadence**, not the rhythm players feel
inside a room. Escape rooms keep their own rhythm through scenes, beats, clocks,
hint timing, reveals, pressure, relief, and reset. Use `docs/scene-beat-harness.md`
to design room rhythm.

This spec borrows the wave/pulse cadence from peer-tracker-style skill
development:

| Term | AMAZE meaning |
|---|---|
| Phase | The puzzle-type family currently being developed, such as signal, tactile, route, mechanical, light, audio, classification, or collaboration. |
| Wave | A batch of AMAZE design work in that phase, usually one signature candidate family plus support variants. |
| Pulse | A short AMAZE design-test cycle: card candidates, simulate with personas, score, log surprises, revise, then lock or park. |
| Base skill | The player skill the phase teaches and tests, such as observation, ordering, extraction, spatial reasoning, audio reasoning, tactile reasoning, or collaboration. |
| Room rhythm | The player-facing pacing of a specific room: scenes, beats, clocks, transitions, reveals, hint pressure, and emotional contour. |

## Phase workflow

1. Pick the current puzzle-type phase.
2. Name the base skills it teaches and tests.
3. Choose theme families where the puzzle type belongs naturally.
4. Generate a wave of hopper candidates.
5. Run one pulse: card, physicalize, simulate, score, and revise.
6. Promote only candidates that pass safety, accessibility, ops, build, and
   evidence gates.

Avoid using a code system only because it is famous. Morse, Braille, semaphore,
binary, and ciphers need in-room teaching, physical action, and a reason to
belong to the story.

## Phase roadmap

| Phase | Puzzle types | Base skills | Physical expressions | Good theme families | First design-pulse test |
|---|---|---|---|---|---|
| 1. Signal and notation | Morse, semaphore, flag codes, call signs, binary, beacon intervals, road signs | observation, ordering, extraction, audio reasoning | radio tuning, light pulses, knock patterns, switch banks, signal flags, dashboard indicators | pirate radio, storm beacon, spy van, roadside observatory, lost expedition | Can novice teams learn the notation in-room within 3 minutes? |
| 2. Tactile and raised pattern | Braille-like cells, embossed maps, textures, key cuts, notch patterns, pressure grids | tactile reasoning, observation, indexing, spatial reasoning | raised labels, touch panels, route plaques, textured objects, peg boards, cabinet pulls | archive for the dark, repair shop, accessibility-forward mystery, field station, artifact lab | Can teams solve by touch/shape without fine-motor frustration? |
| 3. Route and spatial transform | maps, compass bearings, overlays, folding, rotation, sightlines, parallax, scale | spatial reasoning, ordering, transformation, indexing | windows, fold-down tables, floor tracks, cabinets, route maps, mirrors, trailer orientation | road trip anomaly, mobile cartography lab, smuggler route, surveyor trailer, time-tour office | Do players know where to stand, align, or rotate? |
| 4. Mechanical state | latches, locks, magnets, weights, balances, valves, switches, gears, cables | tactile reasoning, cause/effect, ordering, collaboration | shore-power panels, galley storage, table hinges, cabinet locks, ballast boxes, service hatches | repair crew, mobile workshop, carnival machine, submarine trailer, haunted appliance | Can operators detect jams, force attempts, and bypass failures? |
| 5. Light and reveal | UV, shadows, filters, mirrors, lenses, color mixing, reflection, silhouettes | observation, transformation, spatial reasoning, extraction | awnings, windows, lanterns, cabinets, inspection lights, star fields, projection boxes | darkroom, lighthouse, meteor lab, magic show, desert observatory | Is the reveal readable under real venue lighting? |
| 6. Audio and rhythm | pitch, tempo, silence, voice fragments, call-and-response, radio static, knocks | audio reasoning, ordering, extraction, collaboration | radio, intercom, speaker grille, pipes, metronome, crank phonograph, phone handset | pirate radio, music trailer, emergency dispatch, haunted broadcast, roadside diner | Can players distinguish signal from ambience? |
| 7. Classification and indexing | sorting, field guides, inventories, taxonomies, ownership, chronology, material groups | classification, association, indexing, extraction | specimen drawers, postcards, receipts, tool racks, galley inventory, luggage tags | museum trailer, naturalist lab, lost-and-found office, traveling library, evidence van | Can teams explain why the chosen group is correct? |
| 8. Collaboration and convergence | asymmetric info, two-person sync, separated viewpoints, metapuzzle, role handoffs | collaboration, metapuzzle, communication, role coverage | opposite windows, linked controls, callouts, synchronized switches, final console | mission control, ritual wagon, heist van, rescue trailer, expedition base | Does every player have meaningful work at the final beat? |

## Pulse cadence

Run each phase through small AMAZE design pulses instead of designing a whole
room at once. A pulse may produce room candidates, but it is not a room pacing
unit.

| Pulse | Output | Stop condition |
|---|---|---|
| Pulse 1: seed | 6-10 raw puzzle-type candidates and 2-3 theme families. | Every candidate names an aha and physical action. |
| Pulse 2: card | 3-5 complete hopper cards. | Inputs, transform, output, clue sources, red herrings, failure mode, and reset state are visible. |
| Pulse 3: physicalize | 2-3 mechanism sketches with build risks and accessibility alternatives. | No required beat depends on pure paper decoding. |
| Pulse 4: simulate | Scene/beat run with at least three personas and one operator stress pass. | Stuck states, hint triggers, reset effects, and safety flags are logged. |
| Pulse 5: promote | 1-2 candidates enter a room graph or the phase parks. | Promotion gates pass with evidence, not author confidence. |

## Boundary with room rhythm

Do not use phase, wave, or pulse to describe what players experience during a
session. Use these room-rhythm terms instead:

| Room-rhythm term | Meaning |
|---|---|
| Scene | A physical zone or phase of play inside the room. |
| Beat | A playable moment: discovery, aha, action, reveal, transition, stall, hint, or payoff. |
| Clock | A pressure track: session time, soft warning, hard exit, reset, or safety. |
| Transition | How players move from one scene, state, or puzzle focus to another. |
| Reveal | A change in player understanding, room state, sensory layer, or story stakes. |
| Relief | A designed breath after pressure, confusion, darkness, sound, or intense collaboration. |

Design cadence asks, "What puzzle type are we developing this week?" Room rhythm
asks, "What does the team feel minute by minute?"

## Code and notation guidance

| System | Phase | Use when | Avoid when | Fair physical teaching |
|---|---|---|---|---|
| Morse | Signal and notation | The fiction includes radios, beacons, telegraphy, emergency signals, or repeated pulses. | Players must recognize Morse from memory or listen under noisy conditions. | Include a visible key, repeated short examples, light or tactile pulses, and an operator-visible stuck signal. |
| Braille-like cells | Tactile and raised pattern | The room uses touch, raised maps, accessibility themes, darkness alternatives, or encoded surface patterns. | Real Braille literacy is required, or blind accessibility is treated as decoration. | Use a diegetic legend, consistent six-cell shapes, large durable raised dots, and a visual/tactile alternative. |
| Semaphore/flags | Signal and notation | The room has sightlines, windows, route signals, maritime/rail/roadside themes, or body-position play. | Players must perform large unsafe motions in a cramped aisle. | Use mounted arms, small flags, silhouettes, or window markers with a clear alphabet subset. |
| Binary | Mechanical state | The theme has power states, switches, lights, diagnostics, or on/off machinery. | It becomes abstract math disconnected from the room. | Teach with labeled examples, group bits in small chunks, and let switches or lights embody state. |
| Substitution ciphers | Classification and indexing | The fiction includes archives, spies, field manuals, ownership marks, or alien/occult scripts. | The cipher is a paper worksheet with no trailer-scale action. | Tie symbol substitution to props, routes, surfaces, or mechanical alignment. |
| Coordinates | Route and spatial transform | The room has maps, stars, routes, seats, cabinets, drawers, or physical grids. | Coordinates point to tiny hidden objects with no clue boundary. | Make the grid large, labeled, reachable, and verifiable from multiple positions. |
| Color codes | Light and reveal | The room has wiring, lights, filters, paint, signage, or sorting. | Color is required without accessibility alternatives. | Pair color with position, symbol, texture, or label. |
| Music notation | Audio and rhythm | The fiction has songs, jukeboxes, recordings, bells, or rhythm devices. | Reading sheet music is required. | Use contour, count, pitch height, repeated motifs, or playable objects instead of formal literacy. |

## Theme families by phase fit

| Theme family | Primary phases | Signature moment hypothesis |
|---|---|---|
| Pirate radio trailer | Signal and notation; audio and rhythm; route and spatial transform | Players tune a forbidden broadcast by aligning route, frequency, and pulse pattern. |
| Storm beacon rescue | Signal and notation; light and reveal; collaboration and convergence | Players restore a mobile beacon by relaying light/Morse across windows and controls. |
| Archive for the dark | Tactile and raised pattern; classification and indexing; light and reveal | Players read a raised archive by touch, then reveal why the labels changed. |
| Roadside observatory | Route and spatial transform; light and reveal; audio and rhythm | Players rotate the trailer's interior into a star map that unlocks a signal. |
| Mobile repair workshop | Mechanical state; binary; classification and indexing | Players diagnose a broken trailer by setting safe power, valves, and service panels. |
| Evidence van | Classification and indexing; substitution; collaboration and convergence | Players connect physical evidence categories into a final suspect route. |
| Traveling library | Classification and indexing; tactile pattern; substitution ciphers | Players shelve, stamp, and index books or cards until the room reveals a hidden catalog. |
| Heist support van | Collaboration and convergence; signal and notation; mechanical state | Players coordinate tools, lookout signals, and timed mechanisms without splitting the party unsafely. |
| Desert survey trailer | Route and spatial transform; coordinates; light and reveal | Players use maps, shadows, and measurement tools to locate an impossible landmark. |
| Haunted broadcast diner | Audio and rhythm; signal and notation; classification and indexing | Players sort orders, voices, and radio fragments into the broadcast's missing message. |

## Phase mix rules

| Rule | Reason |
|---|---|
| One signature phase, two support phases maximum | More than three mechanic families usually hides difficulty and increases reset risk. |
| Do not stack two literacy-heavy systems in one required beat | Morse plus cipher plus extraction is usually under-clued. |
| Pair every sensory channel with an alternative | Audio, color, low light, and tactile-only beats need parallel access paths. |
| Mechanic teaches before mechanic tests | The room should demonstrate its notation at low stakes before a required solve. |
| Physical action must express the transform | If players decode on paper and then type a code, the phase is not physical enough. |
| Final convergence should recombine, not introduce a new code | The last beat should pay off learned systems rather than teaching another alphabet. |

## Phase candidate table

Use this table when seeding a room hopper.

| ID | Phase | Design wave | Puzzle type | Base skills | Theme fit | Aha | Physical action | Accessibility alternative | Build risk | First design-pulse test |
|---|---|---|---|---|---|---|---|---|---|---|
