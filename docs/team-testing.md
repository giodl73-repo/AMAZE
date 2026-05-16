# Team Testing Harness

AMAZE tests whole-team dynamics as first-class escape-room inputs. Individual
personas and behavior probes are not enough: many room failures happen in the
space between players.

Use this harness to ask, "What kind of team is in the trailer, how do they
coordinate, and what does the room do to them?"

## Core layers

| Layer | Meaning |
|---|---|
| Behavior | A repeatable player or staff pattern, such as overthinking, forceful handling, or hint avoidance. |
| Persona | A person or role lens used inside a team, such as anxious participant, scout, mechanic, or floor operator. |
| Team archetype | A whole-party dynamic used to test cooperation, conflict, pacing, role coverage, trust, and fun. |
| Team probe | A scene, beat, or operator intervention that tests how the group behaves together. |
| Team outcome | The observed effect: shared aha, one-player dominance, conflict, delight, shutdown, recovery, or reset damage. |

## Required team archetypes

Run every serious room concept against these archetypes before build readiness.

| Team archetype | What it tests | Common signals | Design response |
|---|---|---|---|
| Amazing team | Best-case group: curious, collaborative, observant, playful, resilient. | Shared discoveries, good handoffs, quick clue uptake, memorable recap. | Check that the room can sing when players are good; do not over-hint or over-linearize. |
| Confused team | Misses early assumptions, misreads clue hierarchy, or cannot find the room's language. | Repeated wrong theories, silence, wandering, asking "what are we supposed to do?" | Stronger onboarding, clearer affordances, low-stakes teaching beat, attention-first hints. |
| Fighting team | Social friction, dominance, frustration, blame, or competing theories. | Interruptions, one player grabs props, visible annoyance, hint shame risk. | Parallel roles, neutral hint wording, tasks that require handoff not argument, operator morale tools. |
| Speed team | Experienced players moving fast, pattern-matching, brute-forcing, skipping story. | Early lock attempts, sequence breaks, ignored narration, final bypass attempts. | Anti-bruteforce design, story-carrying mechanisms, optional expert shortcuts that preserve signature beat. |
| Quiet team | Low talk, low confidence, hint hesitation, private solving. | Long silences, separated attention, no one asks for help, slow convergence. | Visible progress feedback, prompts that create table talk, gentle operator check-ins, shared artifacts. |
| Chaotic team | Excited, messy, touches everything, moves props, loses state. | Props scattered, simultaneous attempts, reset risk, accidental reveals. | Bounded search zones, durable props, reset inventory, physical state indicators, staff-visible misuse. |
| Family/mixed team | Mixed ages, reach, reading load, patience, confidence, and attention. | Adults dominate, kids handle fragile parts, crowding, uneven participation. | Kid-safe parallel tasks, clear roles, reach alternatives, robust components, visual/tactile clues. |
| Anxious/trust team | Lower tolerance for confinement, darkness, noise, time pressure, or embarrassment. | Exit checking, distress, withdrawal, sensory overload, hint reluctance. | Visible exit trust, opt-out path, sensory alternatives, pressure relief, dignity-preserving hints. |
| Accessibility-varied team | Different mobility, sensory, language, cognitive, or fine-motor needs. | Required beat excludes someone, unreadable clue channel, inaccessible reach. | Multimodal clues, seated participation, reach checks, alternate interaction paths, staff support. |
| Late/tired team | Low energy, late arrival, distracted group, reduced patience. | Skips briefing, misses setup, morale dips, hard-exit pressure. | Short onboarding recovery, acceleration path, clear clocks, graceful ending, reset-safe shortcuts. |

## Team test matrix

Use this matrix in `SCENES.md` and record outcomes in `PLAYTEST.md`.

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|

## Team probe examples

| Team archetype | Probe |
|---|---|
| Amazing team | Can the room produce a signature moment without operator help, and do players describe it afterward? |
| Confused team | If they miss the first teaching clue, what second signal tells them what kind of room this is? |
| Fighting team | When two theories compete, does the room provide evidence that resolves the argument without shaming either player? |
| Speed team | Can they brute-force the final output before completing the physical signature interaction? |
| Quiet team | Which artifact forces useful table talk without making anyone perform socially? |
| Chaotic team | After all movable props are handled, can staff still verify reset state quickly? |
| Family/mixed team | Does a shorter or younger player have a meaningful safe action in each major scene? |
| Anxious/trust team | At the first pressure increase, can the player see how to pause, exit, or ask for reassurance? |
| Accessibility-varied team | Can required progress happen without relying only on color, hearing, standing reach, or fine motor precision? |
| Late/tired team | Can the operator compress the intro and still preserve the story contract? |

## Minimum team coverage

Before a room can be build-candidate, simulate at least:

1. **Amazing team** - proves the intended best-case fun exists.
2. **Confused team** - proves onboarding, clue hierarchy, and recovery work.
3. **Fighting or quiet team** - proves social dynamics do not break the room.
4. **Speed or chaotic team** - proves bypass, durability, and reset can survive.
5. **Anxious or accessibility-varied team** - proves safety, trust, and participation are real.
6. **Operator stress team** - proves staff can monitor, hint, recover, and hard-exit.

## Team severity

| Severity | Meaning | Response |
|---|---|---|
| Watch | The team dynamic appeared but did not hurt fun, fairness, safety, or reset. | Keep observing. |
| Revise | The dynamic caused confusion, exclusion, delay, conflict, operator friction, or reset risk. | Update scene, beat, hint, physical mechanism, or ops protocol. |
| Block | The dynamic created unsafe pressure, broken trust, inaccessible required progress, or unrecoverable failure. | Do not promote until redesigned. |

## Team-driven review questions

1. What happens when the team is amazing?
2. What happens when the team is confused for the first five minutes?
3. What happens when two players disagree and both feel right?
4. What happens when one player dominates the room?
5. What happens when no one wants to speak up?
6. What happens when players touch everything?
7. What happens when players move too fast for the story?
8. What happens when someone needs to pause or exit?
9. What happens when staff must intervene without embarrassing the team?
