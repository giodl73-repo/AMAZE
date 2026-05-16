# Signal in the Silverstream - Floorplan

Draft floorplan assumptions only. Do not treat these as final measurements or a
specific trailer model.

## Envelope

| Field | Value |
|---|---|
| Trailer model/envelope | Airstream-size trailer or larger |
| Interior length | TBD |
| Interior width | TBD |
| Interior height | TBD |
| Door/threshold constraints | TBD |
| Power location | Shore-power prop must be isolated from real power |
| Operator location | Outside trailer with audio/video monitoring and manual override |
| Maximum team size | 5 |

## Zone map

| Zone | Dimensions | Purpose | Player capacity | Key props/mechanisms | Staff access |
|---|---|---|---:|---|---|
| Entry/briefing | TBD | establish repair-crew premise and visible exit trust | 5 | dispatch intro, exit reassurance | door-side access |
| Route wall | TBD | route reconstruction | 2-3 | postcards, wall map, mileage plaques | reachable from aisle |
| Service/power panel | TBD | safe power sequence | 1-2 active, others observing | low-voltage breaker prop | operator manual trigger |
| Galley cabinet | TBD | frequency extraction | 2 active | UV-lit cabinet, object inventory | reset bin access |
| Fold-down table | TBD | spatial orientation and bearing word | 2 active, 3 observing | rotating/latching table | latch access required |
| Radio/dashboard | TBD | final convergence | 2 active, full team watching | tactile radio dial, audio playback | operator playback bypass |

## Flow

| Moment | Player movement | Gathering point | Crowding risk | Mitigation |
|---|---|---|---|---|
| Arrival | enter, orient to map and trailer systems | entry/route wall | doorway cluster | intro directs team inward and keeps exit visible |
| Route reconstruction | split between postcards and map | route wall | map blocks aisle | map mounted at shared viewing height |
| Power and galley | mechanic at panel, scout/scribe at galley | galley/service area | narrow aisle pinch | make P3 partially parallel after P2 starts |
| Table orientation | team gathers around table | fold-down table | table may block circulation | table movement must not block egress |
| Final broadcast | gather at radio/dashboard | radio zone | everyone crowds final device | final audio should be audible/visible from aisle |

## Egress and safety paths

| Path | Use | Player-visible? | Staff override | Blockers to prevent |
|---|---|---|---|---|
| Normal exit | Session end or voluntary exit | yes | operator can open/stop session | table or players blocking aisle |
| Emergency exit | Safety exit | yes or clearly reassured | operator hard stop and extraction script | puzzle state locking exit |
| Staff access | Intervention/reset | no need for players | operator kit and manual triggers | hidden components unreachable |

## Operator sightlines

| Zone/beat | Visible to operator? | Monitoring method | Blind spot | Compensation |
|---|---|---|---|---|
| P1 route wall | yes | camera/audio | postcard handling detail | reset count and hint trigger |
| P2 power panel | yes | camera plus panel indicator TBD | false negative if circuit fails | separate operator-visible solved signal |
| P3 galley | partial | camera/audio | missing object may be hidden by bodies | inventory checklist |
| P4 table | yes | camera/audio | latch force detail | mechanical sensor or visible latch mark TBD |
| P5 radio | yes | camera/audio | audio module failure | operator playback bypass |

## Accessibility and reach

| Need | Current path | Open question |
|---|---|---|
| Seated/limited-mobility participation | route, scribe, radio observation, and final reasoning can be seated | confirm reach ranges for panel/table/radio |
| No crawling/climbing required | all required beats are wall, cabinet, table, or dashboard height | verify cabinet height |
| Low-light alternative | UV reveal needs backup labels or operator reveal | design backup that preserves fiction |
| Audio alternative | final broadcast needs caption/card or visual success state | write final transcript card |
| Reach range for required mechanisms | TBD | measure once trailer envelope selected |

## Reset paths

| Reset route | Staff action | Verification point | Time risk |
|---|---|---|---|
| Route wall | shuffle/replace postcards | full card count and clean marks | low |
| Power panel | reset switches | all switches at start state | low |
| Galley | replace objects and reset UV | inventory card complete | medium |
| Table | reset latch/orientation | start mark aligned | medium |
| Radio | reset dial/audio state | station indicator start state | low |

## Transport and mounting notes

| Item/zone | Mounting assumption | Travel risk | Secure/stow plan |
|---|---|---|---|
| route map | mounted flat | peeling/scratches | durable laminate and edge trim |
| postcards | removable | loss/bending | storage pouch with spare set |
| switch panel | hard mounted, low voltage | vibration loosens caps | pre-run switch test |
| galley objects | removable | loss/breakage | padded reset bin |
| rotating table | structural/mechanical | latch drift | travel lock TBD |
| radio module | mounted electronics | knob/audio failure | spare knob and playback bypass |

## Open spatial risks

- Actual trailer dimensions are not declared.
- Table movement must not block egress.
- Galley and power-panel crowding may bottleneck teams of five.
- Reach ranges need validation before playtest.
