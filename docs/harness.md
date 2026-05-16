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

## Current CLI

The first harness lives in `tools/amaze-harness/`.

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- check --room rooms\TEMPLATE
```

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- run --room rooms\TEMPLATE --team "Confused team" --behavior "overthinking" --clock 25
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
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- inventory --query latch
```

## Commands

| Command | Purpose |
|---|---|
| `check --room <path>` | Verifies required room files and key harness headings. |
| `run --room <path> --team <team> [--behavior <behavior>] [--clock <minutes>]` | Reads `SCENES.md` and prints a beat-by-beat run sheet. |
| `optimize --room <path> [--target <minutes>]` | Totals beat timing budgets, flags overrun risk, and recommends hint/acceleration changes. |
| `analyze --room <path> --actual P1=4,P2=5 [--team <team>] [--target <minutes>]` | Compares measured beat times from a simulation/playtest against the timing model. |
| `record --room <path> --run <id> --actual P1=4,P2=5 [--team <team>] [--finish <minutes>] [--hints <count>]` | Appends a run summary and measured beat timing evidence to `PLAYTEST.md`. |
| `matrix --room <path>` | Prints the required team-archetype passes and suggested run commands. |
| `inventory [--query <text>] [--catalog <path>]` | Searches the reusable component catalog for source tiers and price bands. |

## Team run matrix

Use `matrix` to turn `SCENES.md` team archetype probes into an execution queue:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- matrix --room rooms\0001-signal-in-the-silverstream
```

This gives the operator/designer a fast checklist of amazing, confused,
fighting, speed, family/mixed, anxious/trust, and operator-stress passes, with
their probe, observable signal, and suggested behavior focus.

## Inventory lookup

Use `inventory` while drafting a room BOM so puzzle mechanisms stay tied to
repeatable common parts:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- inventory --query audio
```

The command reads `components\INVENTORY.md` by default and prints matching item
IDs, common sources, price bands, durability, and typical uses. Use those IDs in
room `BUILD.md` files before build readiness review.

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
7. record timing evidence in `PLAYTEST.md`;
8. record behavior and team evidence;
9. score with `scoring/RUBRIC.md`;
10. check promotion gates;
11. emit broader `PLAYTEST.md` entries;
12. compare versions without rewriting old evidence.

Keep the harness boring and explicit. If the operator cannot explain what the
tool did, the tool is too magical.
