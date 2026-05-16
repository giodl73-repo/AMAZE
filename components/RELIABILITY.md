# AMAZE Reliability and Chaos Testing

Reliability is the ability of a room to stay fair, safe, fun, and operable when
real teams behave differently from the ideal solve path.

Use this file with `components/DEVICE-REVIEW.md`, `docs/team-testing.md`, and
`docs/behavior-testing.md`.

## Reliability issue taxonomy

| Issue ID | Issue | Meaning | Typical player symptom | Operator detection | Default response |
|---|---|---|---|---|---|
| REL-FP | False positive | Device accepts an incorrect state. | players skip aha or open early | solved state appears without earned proof | pause, restore state, redesign reject path |
| REL-FN | False negative | Device rejects a correct state. | players repeat correct action and blame hardware | correct inputs visible but no output | acknowledge, bypass, inspect tolerance |
| REL-JAM | Jam/stick | Moving part binds or becomes hard to operate. | players push harder or stop trusting prop | motion incomplete, repeated force | stop force, use bypass, repair/clean |
| REL-DRIFT | Calibration drift | Device threshold moves between runs. | inconsistent success/failure | pre-run test differs from reset photo | recalibrate or simplify mechanism |
| REL-RESET | Reset miss | Staff leaves device in wrong start state. | next team sees solved clue or missing step | reset photo/count mismatch | block next run until corrected |
| REL-LOSS | Loose-part loss | Token/card/object missing, pocketed, or hidden. | solve path becomes impossible or delayed | count fails | use spare, improve home/tether/count |
| REL-BREAK | Component breakage | A physical item cracks, tears, bends, loses adhesion, loses power, or stops behaving as expected. | players lose trust, cannot complete beat, or use unsafe force | visible damage, pre-run test fails, state differs from photo | swap spare, bypass, remove from service, redesign material |
| REL-REPLACE | Slow replacement | Staff has a spare but cannot identify, reach, or install it before the run bogs down. | long pause, immersion break, over-target run | admin recovery exceeds beat recovery window | move spare closer, label pouch, simplify fastener, add bypass |
| REL-FORCE | Force/pinch risk | Players apply unsafe force or reach pinch point. | pulling, twisting, leaning, finger exposure | visible prop stress or unsafe posture | freeze play, no-force script, redesign stop |
| REL-SENSORY | Sensory dependence | Required progress depends on hearing, color, darkness, subtle light, or tiny marks. | repeated replay/squinting/wrong color reads | accessibility probe fails | add equivalent channel or redesign |
| REL-STATE | State ambiguity | Players/staff cannot tell if device accepted state. | repeated solved steps, debate, distrust | operator cannot see state | add public indicator/reset photo |
| REL-OPS | Operator invisibility | Staff cannot monitor, hint, or recover state. | late intervention or wrong hint | camera/sightline lacks proof state | move state indicator or add ops marker |
| REL-TRANSPORT | Transport damage | Device shifts/breaks between venues. | first run after move fails | post-transport check differs | case/pad/lock module; retest |
| REL-SOUP | Search/prop soup | Too many loose or similar objects obscure purpose. | rummaging, dumping, inventory fatigue | props outside homes | reduce count, add silhouettes/categories |
| REL-SOCIAL | Social monopoly/conflict | One player dominates or group conflict blocks progress. | others idle, argument, shame | operator hears one voice/raised tension | role split, neutral hint, physical handoff |

## Chaos behavior probes

Use chaos probes deliberately during bench tests and prototype runs. Do not wait
for public players to discover these failures.

| Probe ID | Probe | What to do | Watch for | Passing design response |
|---|---|---|---|---|
| CHAOS-DUMP | Dump loose props | Put all movable station parts on the surface/floor. | missing state, hidden proofs, reset confusion | count card, bounded homes, staff-visible missing state |
| CHAOS-POCKET | Pocket/remove token | Hide one critical token/card/object. | impossible solve, late detection | duplicate staff set and reset count catch it |
| CHAOS-FORCE | Pull/twist/push | Apply plausible public-use force without tools. | false open, pinch, breakage | no false state; no-force script and stop survive |
| CHAOS-GUESS | Random final attempts | Try wrong tokens/codes/switch orders early. | sequence break, final false positive | wrong attempts reject visibly and safely |
| CHAOS-RAPID | Rapid toggling/replay | Flip switches, spin dials, replay audio quickly. | state drift, electronics fault, annoyance | debounce/lockout/manual fallback or passive path |
| CHAOS-SWAP | Swap similar items | Exchange labels, overlays, tokens, or object homes. | wrong clue state or impossible reset | keyed homes, orientation marks, reset photo |
| CHAOS-CROWD | Crowd final device | Let multiple players crowd around finale. | blocked sightlines, one-player domination, egress issue | role lane, caller/checker/handler prompts |
| CHAOS-IGNORE | Ignore instructions | Start touching before intro/category frame lands. | unsafe handling, search soup | visible affordances and first safe action |

## Variability matrix

Every build-serious room should name how each required beat changes under
different teams.

| Team/behavior | What changes | Device/beat risk | Required mitigation |
|---|---|---|---|
| Amazing/speed | players move before story finishes | sequence break, thin delight | anti-bruteforce gate and optional reward |
| Confused/quiet | players miss category language | search soup, private solving | teaching beat, public progress artifact |
| Fighting/dominant | one player controls reasoning or props | social monopoly | physical role split and neutral evidence |
| Chaotic/family | props move fast and roughly | loss, force, reset miss | oversized parts, homes, count cards, stops |
| Accessibility/anxious | sensory/reach/trust assumptions fail | exclusion or distress | multimodal path, visible exit, seated role |
| Late/tired/operator stress | staff compresses or recovers flow | bad hint, reset overrun | acceleration script and reset photos |

## Device FMEA-lite

Use this table in room `PLAYTEST.md` or `BUILD.md` for any yellow/orange/red
device.

| Field | Question |
|---|---|
| Failure mode | What exactly can fail? |
| Cause | Why would it happen: player, device, reset, transport, environment, staff? |
| Player impact | What does the team experience? |
| Operator detection | How will staff know quickly? |
| Bypass | What keeps the run moving? |
| Post-run fix | What repair or redesign happens after the run? |
| Spare/kit | Which spare or kit item supports recovery? |
| Redesign threshold | What observed result means patching is not enough? |

## Criticality and replacement standard

Every critical component in a room `BUILD.md` should declare build time,
replacement time, criticality, and spare count. Treat replacement as an
operator-time problem, not just an inventory problem.

| Criticality | Meaning | Spare rule | Admin recovery target |
|---|---|---|---|
| C1 cosmetic | Nice scenic layer; room still works if removed. | 0-1 spare or repair material. | post-run only |
| C2 helper | Supports clarity or delight but has a hint/bypass. | 1 spare if high-touch or consumable. | 2-5 min or post-run |
| C3 beat-support | Delays or confuses one beat if missing. | 1 ready spare or printed backup. | 1-3 min |
| C4 required | Required for a solve path, but staff can substitute. | 1 installed plus 1 duplicate; stored at operator position or station. | 1-2 min |
| C5 showstopper | Blocks the room, final reveal, safety, or egress if failed. | 2 independent recovery paths: spare plus bypass, duplicate, or manual release. | immediate to 1 min |

Use the highest criticality that applies. A funny crowd-pleaser can still be C5
if the room cannot proceed when it breaks.

## Admin replacement test

For each C3-C5 item, run the replacement drill:

| Step | Pass condition |
|---|---|
| Trigger | Operator can identify the failed/missing item from camera, count card, reset photo, or player report. |
| Locate | Spare is labeled and reachable without searching a mixed bin. |
| Swap/bypass | Staff can replace, hand in duplicate, or trigger bypass without tools unless `OPS.md` says a tool is staged. |
| Recover | Team gets a fair explanation or in-world handoff and can continue. |
| Log | Staff records item, cause, recovery time, and whether the room stayed on target. |

If the admin cannot recover within the beat's slow-case slack, increase spare
count, tether the item, reduce fasteners, stage a duplicate, or redesign the
mechanism.

## Bench result states

| Result | Meaning | Promotion effect |
|---|---|---|
| Pass | Required observation succeeded with no safety, fairness, reset, or access concern. | May advance one review level if other evidence passes. |
| Marginal | Core worked, but a repeated signal suggests friction or fragility. | Revise and retest before public playtest. |
| Fail | Safety, false state, inaccessible required progress, or unrecoverable reset occurred. | Blocks promotion; redesign required. |
| Not run | Planned but no evidence yet. | Blocks any claim based on that test. |

## Reliability gates

A room cannot advance to build-candidate when:

1. Any required device has an untested yellow/orange/red risk band.
2. Any required beat lacks a chaos probe.
3. Any required path has no accessibility-equivalent channel.
4. Any final device can false-open, false-accept, or be forced.
5. Staff cannot detect reset misses before the next team enters.
6. Operator bypass depends on designer knowledge not in `OPS.md`.
7. Transport can change device state without a post-transport check.
