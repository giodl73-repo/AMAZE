# Prism Vault Design Plan

## Status

- Lifecycle state: MUDDLE prototype, inspectable aha-beat pass
- Current version: v0.inspect
- Last evidence source: `amaze-prism` Macroquad play review, role review, and inspect-beat harness coverage
- Next gate: role-reviewed native playtest with novice/family/operator personas

## Experience thesis

Prism Vault should feel like a compact, jewel-box escape room: warm foyer,
crisp light machinery, tactile color, mirrored motion, faceted vault, and a
soft garden payoff. The player should not see "generic MUDDLE boxes"; they
should see a room identity made of distinct zones, props, state changes, and
operator-readable reset signals.

## Visual pillars

| Pillar | Player read | Implementation direction |
|---|---|---|
| Warm entry | Invitation, safety, reset premise | Amber lamps, timer, intake mat, reset placard |
| Light path | Cause-and-effect spine | Beam conduit changes from dark -> safe -> coded -> vault-bound |
| Tactile craft | Physicality, not abstract code | Lens detents, slider notches, mirror brackets |
| Faceted payoff | Vault has weight and spectacle | Prism aperture, status lamps, key chute, pulse on open |
| Garden release | Relief after machinery | Ivy, key hook, reset clipboard, victory threshold |

## Differentiation targets

1. Prism nodes should use a warmer/cooler palette than Silverstream: amber,
   violet, cyan, gold, garden green.
2. Every visible prop should have a purpose: clue, state, atmosphere, reset, or
   safety; no unlabeled filler.
3. Solving should visibly increase density and color, not only change text.
4. Operator/reset artifacts should be present in the fiction so the room feels
   runnable ten times a day.
5. The final scene should read as "vault open + garden exit" at a glance.
6. Optional inspections should reward curiosity without mutating puzzle state or
   replacing the main command path.

## Beat sequence

| Beat | Visual focus | Aha | State change |
|---|---|---|---|
| Entry | Amber lamps, locked garden latch, timer | The room ends by recovering a reset key | Beam dark, reset pending |
| Lens | Fresnel lens, brass detents, daylight gauge | Align physical light before trusting color | Beam safe |
| Color | Cyan/amber/violet sliders, frosted strip | Notches translate light into code | Beam coded |
| Mirrors | Six brackets, colored edges, beam stop | Reflect code in order, not by force | Beam vault-bound |
| Vault | Prism aperture, status lamps, chute | Reflected code releases reset key | Vault open, reset key drops |
| Garden | Ivy, key hook, reset clipboard | Escape is also staff-ready reset | Exit escaped |

## Implementation plan

1. Add product-owned Prism frame names for visual nodes: `prism-warm`,
   `prism-cyan`, `prism-violet`, `prism-gold`, `prism-garden`, `prism-locked`,
   and `prism-solved`.
2. Teach the shared Macroquad renderer a generic way to color these frame names
   without hard-coding AMAZE rules.
3. Update Prism visual nodes so each zone/prop carries a distinct frame and
   stateful props change frame as the room advances.
4. Keep the current command path stable; this pass changes presentation and
   evidence, not puzzle order.
5. Raise/maintain the visual-smoke density gate after the pass so style cannot
   regress silently.
6. Add room-local `inspect ...` commands and visible aha prompts so each beat
   explains the clue logic, reset value, or operator purpose behind its props.

## Non-goals

- No real art asset pipeline yet; sprite source names remain metadata until the
  renderer loads actual images.
- No additional puzzles until role review confirms the current chain is legible.
- No shared renderer behavior that only makes sense for AMAZE.

