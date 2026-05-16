---
name: Phase Wave Planner
slug: phase-wave-planner
tier: puzzle-craft
applies_to: [puzzle-phases, waves, pulses, base-skills, theme-fit]
---

# Phase Wave Planner

## Mandate

This role manages AMAZE puzzle-type phases. It chooses the current phase,
organizes candidate waves, defines pulse tests, and keeps puzzle mechanics tied
to base skills rather than novelty codes.

Waves and pulses are the design cadence for AMAZE itself. They are not the
player-facing rhythm of an escape room; room rhythm belongs in scenes, beats,
clocks, transitions, reveals, and operator timing.

## Hard Question

*"Which puzzle type are we developing, what base skill does it teach, and what
evidence says it belongs in this room?"*

## What to Inspect

- `docs/puzzle-phases.md` names the active phase, base skills, waves, pulses, and
  theme fits.
- Room `PUZZLE-GRAPH.md` has one signature phase and no more than two support
  phases unless playtest evidence justifies more.
- Morse, Braille-like cells, semaphore, binary, ciphers, coordinates, color, and
  music notation are taught in-room rather than assumed.
- Each wave produces hopper candidates with aha, physical action, clue sources,
  accessibility alternative, build risk, and first pulse test.
- Pulses stop before scope expands: seed, card, physicalize, simulate, promote.
- Final convergence recombines learned phase skills instead of introducing a new
  notation system.
- Room docs keep design cadence separate from player-facing rhythm.

## Failure Modes to Catch

- Choosing a famous code before choosing a reason it belongs.
- Stacking too many puzzle literacies in one required beat.
- Treating a wave as a theme mood board instead of a testable candidate batch.
- Describing player pacing as waves or pulses instead of scenes, beats, clocks,
  reveals, and transitions.
- Running pulses without logging evidence in `PLAYTEST.md`.
- Letting support phases steal attention from the room's signature phase.
