# Signal in the Silverstream - Safety

Draft safety review for concept simulation. This is not a certification or code
compliance claim.

## Egress

- Normal exit: main trailer door remains usable and visible/reassured.
- Emergency exit: TBD based on selected trailer envelope.
- Staff override: operator can stop game, open/assist exit, trigger manual puzzle bypasses.
- Player-visible reassurance: intro script should state that the repair crew can exit at any time.

## Hazards

| Hazard | Risk | Mitigation | Open question |
|---|---|---|---|
| prop breaker panel | players may infer real electrical interaction | low-voltage isolated prop only; no exposed real wiring | final electrical design |
| UV cabinet | eye/skin exposure or low-light confusion | enclosed/low-power reveal, backup labels, operator control | exact UV hardware |
| rotating table | pinch, force, egress blockage | rounded edges, travel limits, no required force, operator bypass | latch prototype |
| audio final broadcast | sensory overload or distress | volume cap, transcript/visual alternative, operator stop | final audio level |
| crowded aisle | trip/blocking during table/radio beats | zone limits and clear egress path | actual floorplan |

## Accessibility

- Participation without crawling/climbing: required beats should be playable at standing/seated reachable surfaces.
- Low-light alternative: UV clue needs backup label or operator reveal.
- Audio alternative: final broadcast needs transcript, visual confirmation, or caption card.
- Seated/limited-mobility path: route, scribe, inventory comparison, and final reasoning can include seated roles; reach to table/radio must be validated.

## Operator controls

- Monitoring: audio/video monitoring of route wall, panel, galley, table, and radio.
- Emergency stop: operator can pause/end session immediately.
- Communication: operator hint voice framed as dispatch/repair channel.
- Extraction script: "Dispatch is calling a safety stop. The trailer is safe; step toward the marked exit and I will meet you there."

## Safety gate

| Requirement | Pass? | Evidence | Required revision |
|---|---|---|---|
| Normal exit remains usable | TBD | floorplan draft says table must not block egress | verify in actual trailer |
| Emergency exit is visible or clearly reassured | TBD | intro script planned | define selected trailer exit |
| Staff override cannot be blocked by puzzle state | Partial | manual bypasses listed in build file | test controls |
| Electrical/fire risks are mitigated | TBD | prop panel specified low-voltage only | electrical review before build |
| Required play avoids unsafe crawling/climbing/force | Partial | all beats designed at surfaces | validate reach/table force |
| Anxiety, darkness, noise, and pressure risks are bounded | Partial | exit reassurance, audio stop, UV backup planned | write operator script |
| Accessibility alternatives cover required beats | Partial | low-light/audio alternatives identified | build actual alternatives |
