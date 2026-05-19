# AMAZE

**A mobile escape-room design workshop for trailer-scale builds.**

AMAZE designs escape rooms that can fit inside Airstream-size or larger trailers:
portable, inspectable, resettable game spaces where puzzle craft, physical layout,
safety, operations, and build logistics are designed together from the start.

AMAZE belongs to the Games Design family, alongside puzzle hunts, tabletop
adventures, and board-game systems. Its medium is different: the final artifact
is a compact built environment that has to move, reset, survive teams, and keep
players safe.

## Design promise

Every room should be:

1. **Playable** - puzzles produce fair aha moments, not prop-search frustration.
2. **Buildable** - components fit, mount, reset, and travel inside a real trailer.
3. **Safe** - egress, fire, electrical, visibility, accessibility, and operator
   controls are design inputs, not late checklists.
4. **Operable** - staff can reset, monitor, hint, repair, and transport the room.
5. **Memorable** - the space has a reason to exist beyond being a box of locks.

## First target

The first design target is a trailer-scale escape room, roughly Airstream-size or
larger. The repo should treat exact dimensions, axle/weight limits, power
availability, staffing, jurisdictional rules, and venue constraints as explicit
brief variables.

## Repo layout

```text
AMAZE/
  README.md
  CLAUDE.md
  docs/specs/                  design specs and constraints
  docs/specs/design-system.md  principles, invariants, pitfalls, puzzlehunt lessons
  docs/specs/room-lifecycle.md room template contract and evolution gates
  docs/puzzle-hopper.md        puzzle-hunt-grade idea intake and promotion
  docs/puzzle-phases.md        puzzle-type phases, waves, pulses, and theme fits
  docs/team-testing.md         whole-team archetypes for simulation/playtest
  docs/behavior-testing.md     first-class behavior probes for simulation/playtest
  docs/session-duration.md     30/45/60 minute timing profiles and ops slots
  docs/scene-beat-harness.md   D&D-style simulation harness for rooms
  docs/harness.md              Rust CLI harness for fast beat runs
  docs/build-economics.md      BOM, budget, durability, and reliability gates
  scoring/                     escape-room rubric and scoring protocol
  personas/                    player, operator, builder, and venue lenses
  rooms/                       one directory per room concept
  components/                  reusable inventory, sourcing, props, locks, sensors, lighting, scenic modules
  operations/                  reset, staffing, maintenance, transport, and safety checklists
  research/                    escape-room, trailer, accessibility, and build references
  .roles/                      governance and creative review lenses
```

## Initial workflow

```text
BRIEF -> FLOORPLAN -> PUZZLE GRAPH -> SIMULATION -> BOM/BUDGET -> BUILD PLAN -> SAFETY REVIEW -> PLAYTEST -> OPS PACK
```

The workflow is intentionally physical. A puzzle graph that cannot fit in the
room, survive transport, or reset cleanly is not ready.

## Design loop

AMAZE uses the Games Design loop:

```text
DESIGN -> SIMULATE -> SCORE -> LOG SURPRISES -> IMPROVE -> LOCK NEXT VERSION
```

Rooms are scored with [`scoring/RUBRIC.md`](scoring/RUBRIC.md). New puzzle ideas
start from puzzle-type phases in
[`docs/puzzle-phases.md`](docs/puzzle-phases.md), enter the hopper through
[`docs/puzzle-hopper.md`](docs/puzzle-hopper.md), then simulations use the
D&D-style [`docs/scene-beat-harness.md`](docs/scene-beat-harness.md) and personas
in [`personas/`](personas/) to test different teams, staff choices, hint timing,
reset friction, and safety edge cases before a build plan advances.
Whole-team dynamics are tested through
[`docs/team-testing.md`](docs/team-testing.md): amazing teams, confused teams,
fighting teams, quiet teams, chaotic teams, speed teams, family/mixed teams,
trust/anxiety teams, and accessibility-varied teams. Specific player and staff
behaviors are tested through [`docs/behavior-testing.md`](docs/behavior-testing.md).
Fast beat runs and configurable session optimization can be generated with the
Rust CLI described in [`docs/harness.md`](docs/harness.md). Session length is a
room variable: see [`docs/session-duration.md`](docs/session-duration.md) for
30-minute trailer, 45-minute staffed-hour, and 60-minute standard profiles.
Every promoted puzzle beat also needs a physical mechanism, reusable inventory
ID or custom fabrication plan, and build economics review through
[`docs/build-economics.md`](docs/build-economics.md): BOM, supplier class, price
band, durability, reliability, breakage recovery, and replacement plan. Common
parts live in [`components/INVENTORY.md`](components/INVENTORY.md), with sourcing
rules in [`components/SOURCING.md`](components/SOURCING.md).

The repo-level design contract lives in
[`docs/specs/design-system.md`](docs/specs/design-system.md), with room lifecycle,
promotion gates, and template evolution rules in
[`docs/specs/room-lifecycle.md`](docs/specs/room-lifecycle.md). Start new room
folders from [`rooms/TEMPLATE/`](rooms/TEMPLATE/) and keep all eight room
surfaces current as the design moves from seed to simulation, prototype,
playtest, and field-ready versions.

## Current status

Foundation scaffold plus first room seed:
[`rooms/0001-signal-in-the-silverstream/`](rooms/0001-signal-in-the-silverstream/).

## MUDDLE surface API

The harness exposes product-owned escape-room play surfaces for MUDDLE adapter
work:

```rust
let surface = amaze_harness::silverstream_muddle_surface();
let host = amaze_harness::silverstream_muddle_host();
let prism_surface = amaze_harness::prism_vault_muddle_surface();
let prism_host = amaze_harness::prism_vault_muddle_host();
```

The surface provides room ids, exits, resource/status defaults, objectives, and
command hints without moving MUDDLE renderer code into AMAZE. The host implements
MUDDLE's `MuddleHost` contract for stateful Silverstream play, including
checkpoint export/import for the route rail, breaker panel, galley reveal, fold
table, radio broadcast, hatch, and hint state.
Prism Vault adds a second MUDDLE room with lens, color, mirror, vault, and
garden-exit checkpoint state so the shared clients can be tested against
multiple AMAZE hosts.

Run the product-owned MUDDLE launcher from `tools\amaze-harness` with:

```powershell
cargo run --quiet --bin amaze-muddle
cargo run --quiet --bin amaze-muddle -- --save silverstream.muddle
cargo run --quiet --bin amaze-muddle -- --load silverstream.muddle --save silverstream.muddle
cargo run --quiet --bin amaze-muddle -- --transcript silverstream-transcript.txt
cargo run --quiet --bin amaze-prism-muddle
cargo run --quiet --bin amaze-muddle-window -- --open
cargo run --quiet --bin amaze-muddle-window -- --host amaze-prism-vault --open
cargo run --quiet --bin amaze-muddle-window -- --save silverstream.window.muddle --transcript silverstream.window.txt --open
cargo run --quiet --bin amaze-muddle-window -- --load silverstream.window.muddle --save silverstream.window.muddle --transcript silverstream.window.txt --open
cargo run --quiet --bin amaze-muddle-macroquad
cargo run --quiet --bin amaze-muddle-macroquad -- --host amaze-prism-vault
cargo run --quiet --bin amaze-muddle-macroquad -- --save silverstream.macroquad.muddle --transcript silverstream.macroquad.txt
```

The native Macroquad launcher uses AMAZE-level chrome and defaults to the full
Silverstream route-to-radio arc, with Silverstream save/transcript/import/export
paths. Passing `--host amaze-prism-vault` opens Prism Vault with Prism-specific
default save/transcript/import/export paths. The first Silverstream command arc
is: `go route`, `sort postcards`, `go breaker`, `set breakers`, `go galley`,
`sort galley`, `go table`, `align table`, `go radio`, `tune radio`,
`unlock hatch`, `go hatch`.
The Prism Vault command arc is: `go lens`, `align lens`, `go color`,
`mix color`, `go mirrors`, `set mirrors`, `go vault`, `unlock vault`,
`go exit`.

## License

Private repository. License decision pending.
