# Session Duration Profiles

AMAZE treats session length as a design variable, not a fixed rule. The harness
uses `--target <minutes>` so a room can be optimized for a short trailer demo, a
staffed hourly slot, or a standard full escape-room session.

## Common profiles

| Profile | Player time | Ops slot | Best use | Design implication |
|---|---:|---:|---|---|
| Short trailer | 25-30 min | 40-45 min | festivals, previews, small mobile footprint, high throughput | fewer beats, fast onboarding, aggressive hint timing, very tight reset |
| Staffed-hour | 40-45 min | 60 min | mobile bookings where staff sell one-hour blocks | 45 minutes of play plus 10-15 minutes for reset, greeting, waiver, and turnover |
| Standard escape room | 60 min | 75-90 min | traditional escape-room expectation or premium mobile booking | deeper puzzle graph, more parallelism, slower story ramp, more robust hint pacing |
| Uber session | 90 min | 120 min | premium bookings, enthusiast teams, special-event anchor experience | act structure, comfort-pause policy, state cards, role rotation, heavier reset staffing |

The most common escape-room expectation is **about 60 minutes of play**. For
mobile trailer operations, **45 minutes of play inside a one-hour staffed slot**
is often a practical target because it leaves time for reset and turnover.

## Choosing a profile

| Question | Bias |
|---|---|
| Is the product sold as a classic escape room? | 60-minute play profile |
| Is the product sold as a mobile hourly booking? | 45-minute staffed-hour profile |
| Is the product for festivals, previews, or high-throughput events? | 25-30 minute short-trailer profile |
| Is the product a premium long-form anchor experience? | 90-minute uber-session profile with a 120-minute ops slot |
| Is reset complex or prop-heavy? | shorter play or longer ops slot |
| Is the room story-heavy or metapuzzle-heavy? | longer play profile |

## Timing model

For any profile:

1. Declare player time, soft-warning time, hard-exit time, reset target, and ops
   slot in `BRIEF.md` and `OPS.md`.
2. Give every beat `Target min`, `Hint at min`, and `Slow max min` in `SCENES.md`.
3. Run the harness with the matching target:

```powershell
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- optimize --room rooms\<slug> --target 45
```

4. Keep planned beat time below player time to leave room for intro, movement,
   reveals, human variance, and graceful exit.

## Profile targets

| Player time | Suggested beat target total | Slow-case ceiling | Notes |
|---:|---:|---:|---|
| 30 min | 20-23 min | 27-30 min | trailer preview; very little slack for slow teams |
| 45 min | 32-36 min | 40-45 min | strong mobile default for hourly staffing |
| 60 min | 43-50 min | 55-60 min | common classic escape-room expectation |
| 90 min | 65-72 min | 82-90 min | premium long-form session; requires act breaks, role rotation, state cards, and a comfort-pause policy |

If slow-case equals the full player time, the room is playable for simulation
but needs acceleration paths before paid public runs.

## Uber-session design risks

A 90-minute mobile room should be treated as a sequence of acts, not a normal
45-minute room with twice as many puzzles. Every act needs a visible state
change, a role refresh, a reset boundary, and an operator accelerator. Critical
facts should become physical state cards so teams do not need to remember a clue
from an hour earlier. The ops pack must also declare comfort-pause, temperature,
fatigue, staffing, reset, and emergency-interruption paths before paid use.
