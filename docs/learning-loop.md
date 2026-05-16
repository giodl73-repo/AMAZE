# AMAZE Learning Loop, Alternatives, Backups, and Hints

AMAZE rooms improve by preserving evidence, not by trusting designer memory.
Every room should record what we learned, what else we could build, what happens
when a device fails, and what hints staff should give.

## What counts as a learning

| Learning type | Meaning | Where to record |
|---|---|---|
| Player learning | What players did, misunderstood, loved, ignored, forced, or remembered. | `PLAYTEST.md` learning ledger and surprise log |
| Device learning | What failed mechanically, electrically, socially, or under reset. | `BUILD.md`, `PLAYTEST.md`, `components/RELIABILITY.md` |
| Operator learning | What staff could/could not see, hint, bypass, reset, or explain. | `OPS.md`, `PLAYTEST.md` |
| Room-system learning | A lesson that should affect templates, rubric, or shared catalogs. | `rooms/TEMPLATE/`, `docs/`, `components/` |

## Learning ledger

Use this in `PLAYTEST.md` when a result should change future design.

| Field | Question |
|---|---|
| Learning | What did we learn? |
| Evidence | What run, test, score, or observation proves it? |
| Scope | Room-only, device catalog, template, rubric, ops, safety, or sourcing? |
| Decision | Promote, revise, park, retire, template-change, or catalog-change? |
| Follow-up | What exact file, beat, device, or test changes next? |

## Alternatives and backups

An alternative is a different way to create the same player-facing beat. A backup
is what staff does when the chosen implementation fails mid-run.

| Type | Use when | Example |
|---|---|---|
| Build alternative | The aha is good but the device may be too fragile, expensive, unsafe, or hard to reset. | replace sensor detection with a visible keyed socket |
| Play alternative | The team/persona cannot use the primary clue channel. | printed transcript for audio, high-contrast card for UV/light |
| Operator backup | The device fails during a run. | staff proof token, manual release, printed success card |
| Reset backup | Staff cannot restore state fast enough. | reduce token count, fixed plaques, reset photo, duplicate pouch |
| Safety backup | A prop path becomes unsafe. | freeze play, bypass device, remove/tether prop, hard-exit script |

## Alternatives table

Use this in `PUZZLE-GRAPH.md` or `BUILD.md` for build-serious beats.

| Beat/device | Primary implementation | Alternative implementation | What stays the same | Tradeoff | Trigger to switch |
|---|---|---|---|---|---|

## Backup plan table

Use this in `OPS.md` for anything powered, movable, force-prone, reset-heavy, or
accessibility-sensitive.

| Beat/device | Failure or access issue | Player-facing line | Staff backup | Preserves fiction? | Post-run action |
|---|---|---|---|---|---|

## Hint design

Hints should preserve discovery. They should move attention before they reveal
relationships, and reveal relationships before they give an answer.

| Hint level | Purpose | Good hint shape | Avoid |
|---|---|---|---|
| Attention | Point players back to the right object/area/channel. | "The rail wants a stack." | answer fragments |
| Relationship | Clarify how two things connect. | "Match edge shapes before colors." | full solve sequence |
| Acceleration | Keep schedule/safety without pretending players solved unaided. | demonstrate one rejected tile, provide staff proof | silent handwave or unexplained bypass |
| Hard-exit | End safely and gracefully. | "The steward can certify this proof so you can finish the machine." | shame, panic, or fake failure |

## Hint table

Use this in `OPS.md`; `PUZZLE-GRAPH.md` may keep the shorter design-facing hint
ladder.

| Beat/stuck state | Observable signal | Intent | Hint 1: attention | Hint 2: relationship | Acceleration | Do not say |
|---|---|---|---|---|---|---|

## Promotion questions

Before a beat promotes, answer:

1. What did we learn from the last test?
2. What alternative implementation preserves the same aha if this device fails?
3. What is the operator backup during a live run?
4. What hint preserves discovery without hiding the intervention?
5. What evidence would make us switch alternatives, not patch harder?
6. What lesson should move into a shared catalog or template?
