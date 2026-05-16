# AMAZE Harness

The AMAZE harness is a fast, D&D-style beat runner for trailer escape-room
design. It is not a physics simulator and not a replacement for playtests. It is
a command-line way to run scenes and beats quickly against team archetypes,
behavior probes, clocks, hints, mechanism risks, and reset effects.

## Why Rust

The harness should stay fast, deterministic, portable, and easy to run in a
terminal. Rust is a good fit because the tool can become a small compiled binary
that reads room files and produces run sheets without a slow runtime or large
dependency stack.

## Architecture

The harness remains one Rust crate for now. That keeps install/run friction low
while the room model is still changing, but the code is split along stable seams:

| Module | Owns |
|---|---|
| `main.rs` | binary entrypoint, CLI parse call, and process exit handling |
| `cli.rs` | argument parsing, command structs, defaults, and help text |
| `domain.rs` + `domain\*.rs` | named Markdown section loaders, typed row contracts, and scene/build/playtest/evidence/catalog document bundles |
| `evidence.rs` | bench/admin/chaos evidence extraction, filtering, status normalization, and sprint filters |
| `markdown.rs` | Markdown table extraction/scanning, row lookup, minute parsing, section appends, and report-cell escaping |
| `room.rs` | typed room contract issues, required room-file plus exact heading-line checks for every room artifact, active room discovery, and typed room document reads |
| `scenes.rs` | shared `SCENES.md` table semantics, including team-to-behavior lookup |
| `rules\simulation.rs` | team/chaos selection, reliability event chance, component recovery, and build-time variance rules |
| `rules\timing.rs` | timing fit, slack, hint-pressure, actual-time matching, and acceleration rules |
| `rules\unlocks.rs` | transformation-state and unlock-path readiness rules, beat-reference parsing, and coherence field validation |
| `rules\visual.rs` | C4-C5 visual-reference gate, visual readiness rows/filters, matching, and criticality rules |
| `sim.rs` | deterministic RNG, sampling helpers, and simulation math primitives |
| `commands\mod.rs` | command module registry and `Command` dispatch |
| `commands\check.rs` | structured room-contract issue rendering and visual-reference gate wiring |
| `commands\lint.rs` | `mdpath`-backed room Markdown hygiene checks before deeper parsing |
| `commands\catalog.rs` | inventory and beat-package catalog reports |
| `commands\visuals.rs` | C4-C5 visual readiness report and portfolio sprint queue |
| `commands\bench.rs` | bench/admin/chaos evidence report and portfolio sprint queue |
| `commands\score.rs` | draft rubric scorecards for one room or a room portfolio |
| `commands\simulation.rs` | randomized run simulation, reliability/chaos pressure, and build-time pressure report surfaces |
| `commands\timing.rs` | run sheets, session optimization reports, timing analysis reports, playtest recording, and team matrices |

Unit tests live beside the modules they exercise. The typed domain layer should
grow only where repeated rules need stable row contracts. A multi-crate
workspace should wait until those models need reuse outside the CLI.

## Parseable room pack format

An AMAZE room is a folder under `rooms\` that follows the Markdown room-pack
contract. Start from `rooms\TEMPLATE\`, keep the required files, and keep required
section names as exact Markdown heading lines. A heading mentioned only in a
paragraph or table cell does not satisfy the contract.

| File | Harness contract |
|---|---|
| `BRIEF.md` | Required. Contract-checks status, concept, invariants, and success criteria headings. |
| `FLOORPLAN.md` | Required. Contract-checks envelope, zones, flow, egress/safety, operator sightlines, accessibility, reset, transport, and open spatial risks. |
| `PUZZLE-GRAPH.md` | Required. Contract-checks nodes, technique play profile, and edges. |
| `SCENES.md` | Required and parsed. Drives run sheets, team matrices, timing optimization, timing analysis, and randomized simulations. |
| `BUILD.md` | Required and parsed. Drives BOM lookups, visual readiness gates, reliability pressure, component break/loss pressure, spares, replacement timing, and build-time bottlenecks. |
| `SAFETY.md` | Required. Contract-checks egress, hazards, accessibility, operator controls, and safety gate headings. |
| `PLAYTEST.md` | Required and parsed. Stores recorded run evidence and drives variability, bench, admin replacement, chaos, team, behavior, stuck-state, and score reports. |
| `OPS.md` | Required and partially parsed. Contract-checks staffing, run clock, hints, reset, failure modes, transport/maintenance, operator stress tests, NPC/operator voices, and multi-room rotation models. |

The parser currently has typed document bundles for `SCENES.md`, `BUILD.md`,
`PLAYTEST.md`, and catalog files. The remaining room files are already part of
the required room contract, and should become typed only when repeated commands
need stable row data from them.

Validate the room contract with:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- check --room rooms\your-room
```

Lint Markdown structure across room packs with:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- lint --rooms rooms
```

The linter uses the GitHub-hosted `mdpath` dependency as the Markdown parser,
then flags duplicate heading anchors, skipped heading levels, empty headings,
table rows whose cell counts do not match the header, and section-specific
table header contracts for the parsed room surfaces. Failures include `md://`
section locations such as `md://rooms/example/SCENES.md#beat-cards`, so an
editor or future fix tool can jump directly to the section that needs repair.

## Current CLI

The first harness lives in `tools/amaze-harness/`.

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- check --room rooms\TEMPLATE
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- lint --rooms rooms
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- visuals --room rooms\0004-brineworks-at-low-tide
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- visuals --rooms rooms
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- visuals --rooms rooms --open-c5
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- visuals --rooms rooms --status draft --gap pouch
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --kind admin --status not-run
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open --blocker "P2 promotion"
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open --target DEV-SOCKET-001
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- ops --rooms rooms
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- run --room rooms\TEMPLATE --team "Confused team" --behavior "overthinking" --clock 25
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- simulate --room rooms\TEMPLATE --runs 100 --seed 42 --target 45
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- optimize --room rooms\TEMPLATE --target 45
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- analyze --room rooms\TEMPLATE --team "Confused team" --actual P1=5,P2=4,P3=7,P4=8,P5=4 --target 45
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- record --room rooms\TEMPLATE --run SIM-001 --team "Confused team" --personas "First-Date Duo plus quiet observer" --players 2 --finish 38 --hints 3 --actual P1=7,P2=6,P3=8,P4=9,P5=8 --reset 12 --memorable "The trailer answered us." --target 45
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- matrix --room rooms\TEMPLATE
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- score --rooms rooms
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- inventory --query latch
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- packages --query socket
```

## Commands

| Command | Purpose |
|---|---|
| `check --room <path>` | Verifies required room files, key harness headings, visual references for C4-C5 BOM rows, declared transformation/unlock readiness rows, and declared NPC/rotation ops rows. |
| `lint --room <path>` / `lint --rooms <root>` | Uses `mdpath` to lint room Markdown structure before deeper room-specific parsing. |
| `visuals --room <path>` / `visuals --rooms <root>` | Prints C4-C5 visual readiness status, diagrams, and build-readiness gaps for one room or a portfolio of room folders. Add `--open-c5` to show only unresolved C5 showstoppers, `--status draft|blocked|build-ready`, or `--gap <text>` to target repeated blockers. |
| `bench --room <path>` / `bench --rooms <root>` | Prints physical evidence queues from `PLAYTEST.md`: device bench tests, admin replacement drills, and chaos probes. Add `--open`, `--kind bench|admin|chaos`, `--status pending|not-run|passed|failed|blocked`, `--blocker <text>`, or `--target <text>` to plan the next evidence sprint. |
| `ops --room <path>` / `ops --rooms <root>` | Prints NPC/operator voices, multi-room rotation models, shared-operator capacity, and dedicated-staff trigger themes for one room or a room portfolio. |
| `run --room <path> --team <team> [--behavior <behavior>] [--clock <minutes>]` | Reads `SCENES.md` and prints a beat-by-beat run sheet. |
| `simulate --room <path> [--runs <n>] [--seed <n>] [--target <minutes>]` | Runs a deterministic randomized batch across team archetypes, beat timings, hints, chaos probes, reliability pressure, BOM break/loss events, admin replacement recovery, build-time pressure, per-run scores, and average simulation score. |
| `optimize --room <path> [--target <minutes>]` | Totals beat timing budgets, flags overrun risk, and recommends hint/acceleration changes. |
| `analyze --room <path> --actual P1=4,P2=5 [--team <team>] [--target <minutes>]` | Compares measured beat times from a simulation/playtest against the timing model. |
| `record --room <path> --run <id> --actual P1=4,P2=5 [--team <team>] [--finish <minutes>] [--hints <count>]` | Appends a run summary and measured beat timing evidence to `PLAYTEST.md`. |
| `matrix --room <path>` | Prints the required team-archetype passes and suggested run commands. |
| `score --room <path>` / `score --rooms <root>` | Prints a draft 100-point rubric scorecard using room contract, timing, build, playtest, transformation, and unlock evidence. Infers target session minutes from `BRIEF.md` unless `--target` is supplied. Portfolio mode also prints dimension averages and repeated revision pressure. |
| `inventory [--query <text>] [--catalog <path>]` | Searches the reusable component catalog for source tiers and price bands. |
| `packages [--query <text>] [--catalog <path>]` | Searches reusable beat packages that connect techniques, devices, room data, and evidence hooks. |

## Team run matrix

Use `matrix` to turn `SCENES.md` team archetype probes into an execution queue:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- matrix --room rooms\0001-signal-in-the-silverstream
```

This gives the operator/designer a fast checklist of amazing, confused,
fighting, speed, family/mixed, anxious/trust, and operator-stress passes, with
their probe, observable signal, and suggested behavior focus.

## Randomized simulation

Use `simulate` when you want a Monte-Carlo-style pressure test before physical
playtest:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- simulate --room rooms\0004-brineworks-at-low-tide --runs 100 --seed 7 --target 45
```

The command is deterministic for a given seed. It reads:

| Source | Used for |
|---|---|
| `SCENES.md` -> `## Beat cards` | target, hint, slow-case timing, behavior probes, reliability risk text |
| `SCENES.md` -> `## Team archetype probes` | team archetype randomization |
| `SCENES.md` -> `## Behavior probes` | behavior focus per team |
| `SCENES.md` -> `## Transformation states` | visible prop/space phase changes, cabinet/drawer openings, reset states, and failure/bypass notes |
| `SCENES.md` -> `## Unlock paths` | connected-tissue checks across different fast/slow unlock rates |
| `PLAYTEST.md` -> `## Variability matrix` | likely `REL-*` pressure |
| `PLAYTEST.md` -> `## Chaos protocol` | possible `CHAOS-*` events |
| `BUILD.md` -> `## Bill of materials` | component breakage/loss, spare count, replacement minutes, bypass/admin recovery, build-time pressure |

It reports sample runs, average minutes, average hints, over-target rate,
over-slow runs, reliability/chaos events, admin recoveries in time,
late/failed admin recoveries, event pressure, component break/loss pressure, and
build-time bottlenecks. This is a design heuristic, not evidence that replaces
real bench tests, replacement drills, or playtests.

Transformative experiences are modeled in `SCENES.md`, not hidden in build
notes. Use `## Transformation states` to name visible changes such as "cabinet
closed -> cabinet ajar with glowing shelf" and `## Unlock paths` to state how
that new phase remains coherent for a fast team and for a slow team. The
simulation report includes an `Unlock Coherence` section when paths are present,
showing the fast and slow unlock minutes implied by the referenced beat IDs.
`check --room` also validates any declared transformation/unlock rows: required
fields must be non-empty, beat references must match `Beat cards`, and each
unlock path must declare fast coherence, slow coherence, and operator
acceleration.

NPC/operator characters and multi-room rotation are modeled in `OPS.md`. Use
`## NPC and operator characters` for in-world voices that deliver onboarding,
hints, acceleration, safety breaks, and payoff, while declaring limits and
reset/ops burden. Use `## Multi-room operator rotation` for two or three
trailers/rooms sharing an operator; rows must declare the coverage model, maximum
rooms, safe/unsafe conditions, handoff signal, and dedicated-staff trigger.

For admin recovery and build-time modeling, add these optional BOM columns:

| Column | Meaning |
|---|---|
| `Criticality` | C1 cosmetic through C5 showstopper from `components\RELIABILITY.md`. |
| `Build time` | Estimated build hours for installed items and live spares. |
| `Replacement min` | Minutes for staff to replace, hand in duplicate, or bypass. |
| `Spare qty` | Ready live spare quantity; the simulator treats `0`/blank as no spare. |
| `Bypass` / `Admin recovery` | Staff recovery path; a real path can recover even without a spare. |

## Build visual gate

`check` reads `BUILD.md` -> `## Bill of materials` and requires every C4
required or C5 showstopper row to have a usable visual reference. A component
passes if either:

| Evidence | Rule |
|---|---|
| BOM field | `Visual reference`, `Part diagram`, or `Diagram` points to an existing `.excalidraw` file. |
| Part diagrams table | `## Part diagrams` has a matching beat/device row whose `Diagram` points to an existing `.excalidraw` file. |

This keeps build-readiness honest: a C4-C5 part needs at least one editable
visual showing the player-facing part, hidden build, reset state, failure path,
and admin recovery before the room can pass `check`.

`check` also scores visual readiness from the matching `## Part diagrams` row:

| Status | Meaning |
|---|---|
| Build-ready | `What it proves` is filled and `Missing before build readiness` says none/build-ready/complete. |
| Draft | A diagram exists and the remaining room-specific build gap is explicitly named. The room can pass `check`, but the item is not build-ready. |
| Blocked | The diagram row is missing `What it proves` or an explicit `Missing before build readiness` gap. The room fails `check`. |

Use `visuals --room <path>` when you want a clean build-readiness backlog instead
of a pass/fail check. It prints one C4-C5 row per BOM component with the matched
diagram, readiness status, and remaining gap/blocker. Use `visuals --rooms rooms`
to scan all active room folders under `rooms\` and compare the portfolio. The
portfolio report sorts blocked and draft C5 items first, then prints a room
summary, repeated gap themes, and a recommended sprint queue with filter
commands so build-readiness work can be batched. Add `--open-c5` when planning
the next showstopper-only readiness sprint. Add `--status` and `--gap` filters
when batching one repeated blocker across rooms, for example pouch maps or
cabinet dimensions.

## Bench evidence gate

`bench` reads `PLAYTEST.md` evidence tables and turns them into a portfolio queue:

| Source table | Kind | Why it matters |
|---|---|---|
| `## Device bench tests` | `bench` | Proves devices can move from L0 idea to L1 cardboard or later physical confidence. |
| `## Admin replacement drills` | `admin` | Proves staff can recover C4-C5 failures within the declared replacement target. |
| `## Chaos protocol` | `chaos` | Proves force, loss, rapid toggling, guessing, and other stress probes do not break the run. |

Use it after visual readiness is clean:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open
```

The portfolio report includes room summary, kind summary, and a recommended
evidence sprint. A row is considered open unless its status normalizes to
`passed`, `done`, or `complete`; `pending`, `not-run`, `failed`, and `blocked`
stay in the queue.

Use `--blocker` to isolate repeated promotion blockers:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open --kind bench --blocker "P2 promotion"
```

Use `--target` to isolate a device, component, beat, or test ID:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- bench --rooms rooms --open --target DEV-SOCKET-001
```

## Inventory lookup

Use `inventory` while drafting a room BOM so puzzle mechanisms stay tied to
repeatable common parts:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- inventory --query audio
```

The command reads `components\INVENTORY.md` by default and prints matching item
IDs, common sources, price bands, durability, and typical uses. Use those IDs in
room `BUILD.md` files before build readiness review.

## Beat package lookup

Use `packages` before adding custom room-specific puzzle logic:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- packages --query overlay
```

Beat packages live in `components\BEAT-PACKAGES.md`. They sit between technique
IDs and room-specific beats: a room composes packages as data, while the harness
reports whether the expected visual, bench, admin, and chaos evidence exists.
Only add custom harness logic when a repeated custom beat should become a new
package or package rule.

## Session optimization

The optimizer reads `SCENES.md` -> `## Beat cards` and expects these timing
columns:

| Column | Purpose |
|---|---|
| `Target min` | Expected solve time for a healthy run of this beat. |
| `Hint at min` | Minute within the beat when the operator should intervene if no progress is visible. |
| `Slow max min` | Maximum acceptable duration before acceleration or bypass. |

Session length is configurable. The CLI default is 45 minutes because that fits
a common mobile pattern: 45 minutes of play in a one-hour staffed slot, leaving
time for reset and turnover. Use `--target 30` for short trailer/festival runs
or `--target 60` for a standard full escape-room profile.

For any target, the planned beat total should leave buffer for intro,
transitions, final payoff, human variance, and hard exit. A slow-case total over
the target means the room needs earlier hints, acceleration paths, simpler
beats, parallel work, or a cut. See `docs/session-duration.md`.

## Run timing analysis

After a simulation or playtest, pass measured beat times to `analyze`:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- analyze --room rooms\0001-signal-in-the-silverstream --team "Confused team" --actual P1=5,P2=4,P3=7,P4=8,P5=4 --target 30
```

The command reports:

| Signal | Meaning |
|---|---|
| `over target` | The beat took longer than its healthy timing budget. |
| `hint pressure` | The beat reached or exceeded its designed hint minute. |
| `over slow max` | The beat exceeded its maximum duration and needs acceleration, cut, or redesign. |
| `Actual vs planned beats` | Whether the run consumed more or less time than the beat model expected. |
| `Actual vs session target` | Whether the full measured beat total fits the session target. |

Use `record` when the timing evidence should become part of the room history:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- record --room rooms\0001-signal-in-the-silverstream --run SIM-001 --team "Confused team" --personas "First-Date Duo plus quiet observer" --players 2 --finish 25 --hints 3 --actual P1=5,P2=4,P3=6,P4=6,P5=4 --reset 7 --memorable "The trailer answered us." --target 30
```

This appends one row to `PLAYTEST.md` -> `## Simulation and playtest runs` and
per-beat rows to `## Beat timing evidence` without rewriting earlier evidence.

Optional `record` fields include `--version`, `--date`, `--type`, `--personas`,
`--players`, `--finish`, `--hints`, `--stuck`, `--failures`, `--safety`,
`--reset`, and `--memorable`.

## Design target

The harness should eventually support:

1. create a room from `rooms/TEMPLATE/`;
2. run a team archetype against room beats;
3. optimize a target session length from beat timings;
4. analyze measured beat times from simulated or real teams;
5. generate team run matrices;
6. look up reusable inventory, sourcing, and price bands;
7. randomized multi-run pressure tests;
8. record timing evidence in `PLAYTEST.md`;
9. record behavior and team evidence;
10. score with `scoring/RUBRIC.md`;
11. check promotion gates;
12. emit broader `PLAYTEST.md` entries;
13. compare versions without rewriting old evidence.

Keep the harness boring and explicit. If the operator cannot explain what the
tool did, the tool is too magical.
