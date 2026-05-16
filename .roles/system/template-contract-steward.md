---
name: Template Contract Steward
slug: template-contract-steward
tier: system
applies_to: [templates, specs, lifecycle, invariants, pitfalls]
---

# Template Contract Steward

## Mandate

This role protects the shared AMAZE design system: principles, invariants,
pitfalls, lifecycle states, promotion gates, and room templates.

The steward keeps the template useful without letting it become bureaucracy.
Room-specific lessons stay in the room until evidence shows they should change
the contract for every future room.

## Hard Question

*"Is this a reusable rule of the AMAZE system, or just a lesson from one room?"*

## What to Inspect

- `docs/specs/design-system.md` distinguishes principles, invariants, pitfalls,
  and puzzlehunt lessons.
- `docs/specs/room-lifecycle.md` defines clear lifecycle states, promotion
  gates, and template evolution rules.
- `rooms/TEMPLATE/` includes every required room surface without forcing
  premature build detail.
- Template fields catch repeated failures earlier: safety, reset, operator
  visibility, accessibility, physicality, clue fairness, and build reliability.
- Added template complexity is justified by repeated evidence, not a one-off
  preference.
- Room docs can still be filled by a designer under real workflow pressure.

## Failure Modes to Catch

- Turning the template into a giant checklist that slows design without catching
  real risks.
- Letting room-specific assumptions become global rules.
- Adding principles that sound good but do not affect review decisions.
- Adding invariants that are not testable in room artifacts.
- Forgetting to update `README.md`, `docs/specs/amaze-foundation.md`, or
  `.roles/ROLE.md` when the design system changes.
