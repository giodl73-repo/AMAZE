# The Manor in the Mirrorline - Puzzle Graph

This is an original deduction-mystery graph for a spooky manor-in-a-trailer. It
must stay physical: every deduction should come from touching, aligning,
comparing, revealing, weighing, or placing props, not only reading a logic grid.

## Phase mix

| Role | Phase | Base skills | Why it belongs in this room |
|---|---|---|---|
| Signature | classification/indexing | elimination, grouping, association | Culprit/object/location deduction needs visible evidence categories. |
| Support | mechanical state | alignment, setting, restrained handling | The final accusation cabinet makes deduction physical. |
| Support | light/reveal | comparison, hidden marks, transformation | A haunted manor can reveal truth through safe lighting/mirror effects. |

## Design pulse evidence

| Design pulse | Phase | Output | Evidence | Decision |
|---|---|---|---|---|
| v0.theme-lock | classification/indexing, mechanical state | Original culprit/object/location deduction room. | User asked for a Clue-like trailer escape room; we locked original spooky manor tone. | Seed graph and simulate before adding suspects/objects count. |

## Hopper

| ID | State | Phase | Name | Technique/device | Skill tags | Aha | Physical action | Clue sources | Failure mode | Promotion test |
|---|---|---|---|---|---|---|---|---|---|---|
| H1 | promote with prototype | classification/indexing | Portrait truth windows | `TECH-SORT-003`, `TECH-FIT-002` / `DEV-WINDOW-001` | observation, elimination, fit-test | one portrait's alibi token breaks the mirrorline when tested in its own frame | open frames and test alibi tokens in portrait-back truth windows | suspect gallery | token swapping or too much reading | confused team identifies suspect by physical contradiction without hint |
| H2 | promote with prototype | classification/indexing | Object balance plinth | `TECH-FIT-003` / `DEV-BALANCE-001` | sorting, handling, comparison | only one object behaves correctly on the mirrorline balance | test safe prop objects on balance plinth | sideboard, cabinet | prop loss/search soup | chaotic team cannot break or lose critical state |
| H3 | promote with prototype | light/reveal | Room inspection windows | `TECH-ALIGN-001`, `TECH-ALIGN-003` / `DEV-OVERLAY-001` | observation, overlay comparison | miniature room inspection windows show which location refuses the earned suspect/object proofs | align transparent overlays under work light | room stations | glare or swapped overlays | accessibility-varied team gets equal visual path |
| H4 | promote with prototype | mechanical state | Proof-locked mantel mirror | `TECH-TEAM-002`, `TECH-REVEAL-003` / `DEV-FINAL-002` | collaboration, setting, final convergence | proof tokens unlock selector covers, then the team calls/checks/sets the accusation | seat tokens, lift selector covers, set suspect/object/location sliders | all evidence | one-player domination or selector guessing | fighting team uses caller/checker/handler and chaotic team cannot brute-force |
| H5 | park | audio/reveal | Guest bell | `TECH-REVEAL-004` / `DEV-AUDIO-001` | timing, attention | bell rings for truthful category, not ambience | ring/listen/watch indicator | cabinet, operator cue | audio-only clue | visual bell indicator carries same info |
| H6 | promote with prototype | theme/reward | House echoes | `TECH-REVEAL-004` / `DEV-FLAG-001` | optional recall, narrative reward | solved stations leave short physical echo lines that make the final accusation feel haunted | flip optional station backs after proof is earned | station backs, final mirror | reading load or accidental clue confusion | amazing team gets delight without confused team needing it |

## Puzzle review

| Candidate | Verdict | Why it might be fun | Why it might fail | User/design input needed | Next revision |
|---|---|---|---|---|---|
| H1 Portrait truth windows | Promote with prototype | Portraits become active evidence: players open frames, handle alibi tokens, and see one guest physically fail their own story. | If the tokens are too small or the visual line is subtle, it becomes fiddly; if the text is too long, it reverts to a reading puzzle. | P0 suspect set is locked below for prototype only; final names can change after playtest. | Prototype hinged portrait backs with truth windows: four alibi tokens complete a mirrorline, one breaks it and releases the suspect crest. |
| H2 Object balance plinth | Promote with prototype | Handling suspicious manor objects is tactile, social, and easy to understand. The sideboard gives non-readers meaningful work. | Loose objects can become reset debt, and a balance/fit mechanism can become fiddly if tolerances are poor. | P0 object set is locked below for prototype only; every object must be safe, blunt, and oversized. | Prototype a mirrorline balance plinth: only the true object has the right hidden weight/material behavior to level the pointer and reveal the object seal. |
| H3 Room inspection windows | Promote with prototype | Miniature rooms plus transparent inspection overlays can make the trailer feel like a compressed manor. It gives the location category a physical, visual solve. | Can become fiddly if overlays are tiny, glare-prone, or require subtle alignment. Reset can fail if room cards are swapped. | Use high-contrast miniature rooms first: library, greenhouse, observatory, kitchen, parlor. | Prototype work-light inspection windows: suspect crest edge and object seal icon align on all false rooms; only the true room refuses alignment and releases the room-key tile. |
| H4 Proof-locked mantel mirror | Promote with prototype | This is the room's best idea. A physical suspect/object/location accusation gives the whole room a reason to exist and a satisfying final sentence. | If selector covers are cosmetic or can be bypassed, the finale becomes a toy; if covers are too stiff, the climax becomes a maintenance problem. | Make the final cabinet feel like a mantel mirror: central mirror reveal, three proof sockets below, and three selector covers that visibly open when proof is seated. | Prototype mechanical proof gates: crest, seal, and tile each unlock only its category cover; team must call, check, and set before final reveal. |
| H5 Guest bell | Park | A haunted bell is atmospheric and could give the trailer a living-house feel. | As a puzzle it is weak: audio dependence, operator cue risk, and likely ambience confusion. It should not carry required solve information yet. | Do we want the house to "talk" through bell signals at all, or keep it mostly visual/tactile? | Keep as feedback/atmosphere only unless it gains a visual equivalent and a non-audio aha. |
| H6 House echoes | Promote with prototype | Gives the room a remembered emotional texture: each solved station can briefly feel like the manor is admitting something. Fast teams get extra flavor without extra required logic. | If echo copy is too long or too clue-like, players may treat it as required evidence and slow down. | Keep each echo under one sentence and make it appear only after the proof token is earned. | Prototype three station-back echo cards plus one final mirror echo: atmospheric, high-contrast, optional, and resettable. |

## Technique play profile

| Beat/candidate | Technique(s) | Crowd profile | Advantage in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 House ledger intake | `TECH-SORT-001` | onboarding helper | gives deduction categories before players start searching | ledger becomes reading homework | three category plaques and quick operator category language |
| P2 Portrait truth windows | `TECH-SORT-003`, `TECH-FIT-002` | deduction pleaser with fiddly risk | alibis become a group-visible physical contradiction | subtle linework, token swaps, or too much portrait text | bold mirrorline marks, keyed token homes, one worked true alibi |
| P3 Object sideboard | `TECH-FIT-003` | discovery pleaser | object truth is proven by behavior instead of hidden marks | balance drift, false positives, or object search soup | wide tolerance gap, counted tray, visible pointer, staff seal fallback |
| P4 Room inspection windows | `TECH-ALIGN-001`, `TECH-ALIGN-003` | aha pleaser with accessibility risk | miniature rooms and overlays make location physical and spatial | glare, parallax, private squinting, or swapped overlays | work-light route, corner keys, high-contrast marks, duplicate printed card |
| P5 Proof-locked mantel mirror | `TECH-TEAM-001`, `TECH-TEAM-002`, `TECH-REVEAL-003` | premium finale pleaser | proof rail and covers turn deduction into a shared accusation ritual | cover force, token dumping, or one-player slider control | shallow rail, keyed sockets, caller/checker/handler roles, staff bypass |
| H6 House echoes | `TECH-REVEAL-004` | delight support | physical reward cards make fast teams remember the manor | echo copy treated as hidden clue text | reveal only post-proof; one sentence; memory-only operator line |

## Current puzzle quality call

The room idea is good; the individual puzzles are not all good yet.

| Beat | Current quality | Editorial note |
|---|---|---|
| P1 House ledger intake | Serviceable | Good onboarding if it teaches categories quickly. Not a puzzle by itself; keep it short. |
| P2 Suspect gallery | Promote with prototype | Now a physical contradiction puzzle: players test portrait alibi tokens in truth windows, and the one broken mirrorline releases the suspect crest. |
| P3 Object sideboard | Promote with prototype | Now a physical behavior puzzle: players test objects on a mirrorline balance plinth, and only the true object levels/releases the object seal. Prototype tolerance and reset. |
| P4 Room stations | Promote with prototype | Now a work-light inspection-window puzzle: transparent overlays compare suspect/object proof marks against miniature rooms, and the one room that refuses alignment releases the room-key tile. |
| P5 Mirrorline accusation | Best puzzle | Now has a concrete anti-guessing structure: proof tokens unlock selector covers before the team can set the final accusation. Prototype cover feel and false-fit resistance first. |
| Optional house echoes | Delight layer | Station-back echo cards and a final mirror line can make the manor feel alive without carrying required solve information. Keep short, physical, and clearly secondary. |

## P0 answer set

P0 uses a fixed original answer set so the first cardboard prototype can test
fairness, reset, and physical deduction. Change the answer only between runs, not
mid-run.

| Category | Candidates | P0 true answer | Physical proof path | Reset note |
|---|---|---|---|---|
| Suspect | Lady Maribel Vane; Orris Pike; Cora Fen; Ivo Quill; Nella Ash | Nella Ash | Nella's alibi token breaks the portrait truth-window mirrorline and releases the suspect crest | crest returns behind Nella portrait; other tokens return to labeled pockets |
| Object | candlestick; letter opener; snuff box; iron key; hand mirror; bell-pull weight | hand mirror | hand mirror is the only object that levels the balance plinth and releases the object seal | hand mirror gets true-object weight/material behavior; false objects visibly fail |
| Room | library; greenhouse; observatory; kitchen; parlor | observatory | observatory inspection window refuses the suspect/object overlay alignment and releases the room-key tile | room-key tile returns to observatory station pocket |
| Finale | Nella Ash / hand mirror / observatory | same | crest, seal, and tile open the three selector covers before final slider setting | answer key lives in operator kit only |

## Proof-token prototype

The proof tokens are not prizes; they are category proofs. Each token should feel
earned at its station, then make the final cabinet feel inevitable.

| Token | Earned at | Physical form | Socket | What prevents guessing? | What makes it satisfying? | Reset check |
|---|---|---|---|---|---|---|
| Suspect proof | P2 Suspect gallery | portrait-shaped crest with one flat edge and one diagonal "alibi cut" | left cabinet socket shaped like a portrait frame | only Nella Ash's portrait alibi token breaks the truth-window mirrorline in P0 | the token visually matches the contradicted portrait frame | crest returned behind Nella portrait; alibi tokens reset to portrait pockets; staff spare in kit |
| Object proof | P3 Object sideboard | small brass-colored object seal or weighted medallion | center cabinet socket with weight/shape key | only the hand mirror balances the mirrorline plinth and releases the seal in P0 | token is earned by a visible physical test, not found as a hidden mark | seal returned to sideboard tray; object count complete; plinth pointer reset |
| Location proof | P4 Room stations | room-key tile with high-contrast observatory silhouette | right cabinet socket shaped like a room doorway | only the observatory inspection window refuses the suspect/object overlay alignment and releases the tile in P0 | tile is earned by a visible comparison under work light, not by darkness | tile returned to observatory station pocket; overlay cards and panel reset photo match |

### Prototype rules

1. Tokens should be large enough not to pocket accidentally.
2. Tokens should be keyed by category, not by exact final answer alone, so players
   understand why there are three different sockets.
3. Tokens must not reveal final answers before the station is solved.
4. The cabinet should accept only the right category token in each socket, but it
   does not need electronics for the first prototype.
5. Operator must be able to see all three sockets from the monitoring position.
6. Selector covers should stay closed until their category proof is seated.

### Prototype test questions

| Test | Pass condition | Failure means |
|---|---|---|
| Confused team | team understands "earn proof, then accuse" without extra explanation | category model is still too abstract |
| Fighting team | caller/checker/handler roles naturally split token seating and selector setting | cabinet still enables one-player domination |
| Chaotic team | wrong token/wrong socket attempts do not damage or reveal anything | socket design is too fragile |
| Accessibility-varied team | seated or low-light-sensitive player can identify token category and socket | visual/tactile language is insufficient |
| Reset drill | all tokens return to homes and staff verifies state under 10 minutes | too many loose parts or ambiguous reset states |

## Optional house echoes

House echoes are physical reward copy, not clues. They should appear only after a
station proof is earned, be short enough to read at a glance, and never introduce
a new deduction requirement.

| Echo point | Physical reveal | Sample copy | Must not |
|---|---|---|---|
| Suspect gallery | small card on the contradicted portrait back | "The frame remembers who looked away." | name a new suspect or add a hidden alibi |
| Object sideboard | underside card on the balance plinth or seal home | "The sideboard keeps weight better than secrets." | imply a second object test |
| Room stations | small room-panel back card after tile release | "The house folds around the room that will not agree." | add another room exclusion |
| Final mirror | short strip or reflected card after correct accusation | "The manor exhales. For one minute, it is a house again." | require audio, darkness, or exact reading |

## Nodes

| ID | Beat | Technique/device | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|---|
| P1 | House ledger intake | `TECH-SORT-001` / printed frame | observation, role assignment | house ledger, entry bell, visible exit | intro and visible stations | the trailer contains compressed manor rooms | investigation roles and first category rule | reset ledger and bell |
| P2 | Suspect gallery | `TECH-SORT-003`, `TECH-FIT-002` / `DEV-WINDOW-001` | observation, elimination, fit-test | portrait frames, nameplates, physical alibi tokens, portrait-back truth windows, suspect crest token | category rule, P0 portraits | four alibi tokens complete the mirrorline; Nella Ash's own alibi breaks it | suspect proof token | reset portrait windows, alibi tokens, and crest |
| P3 | Object sideboard | `TECH-FIT-003` / `DEV-BALANCE-001` | sorting, tactile comparison, balance | safe prop objects, counted tray, mirrorline balance plinth, object seal token | suspect clues and P0 object set | the hand mirror is the only object that balances the mirrorline | object proof token | replace objects, reset plinth, and return seal |
| P4 | Room stations | `TECH-ALIGN-001`, `TECH-ALIGN-003` / `DEV-OVERLAY-001` | spatial reasoning, overlay comparison | miniature room panels, work-light inspection window, transparent suspect/object overlays, room-key tile | suspect crest edge and object seal icon | false rooms align with the proof overlays; the observatory refuses alignment | location proof token | reset station panels, overlays, and tile |
| P5 | Mirrorline accusation | `TECH-TEAM-001`, `TECH-TEAM-002`, `TECH-REVEAL-003` / `DEV-FINAL-002` | collaboration, metapuzzle, mechanical state | mantel-mirror cabinet with three proof-token sockets, selector covers, accusation sliders, and mirror reveal | suspect, object, location proof tokens | proof opens the category cover; the final answer can only be set after evidence is seated | final reveal and exit token | reset tokens, covers, sliders, and reveal |

## Edges

| From | To | Dependency | Failure fallback |
|---|---|---|---|
| P1 | P2 | ledger teaches categories and portrait rule | operator points to first category mark |
| P1 | P3 | object sideboard can be inspected after role setup | tray count card and plinth icon reduce search |
| P2 | P4 | suspect crest edge becomes one of the inspection-window overlays | portrait back gives one room exclusion |
| P3 | P4 | object seal/material helps read room-station exclusions | seal icon duplicated by printed/high-contrast mark |
| P2 | P5 | suspect proof token opens the suspect selector cover | operator confirms category and places temporary staff proof token |
| P3 | P5 | object proof token opens the object selector cover | operator confirms category and places temporary staff proof token |
| P4 | P5 | location proof token opens the room selector cover | hard-exit acceleration gives one exclusion and temporary staff proof token |

## Hint ladder

| Stuck state | Observable signal | Hint 1 | Hint 2 | Acceleration |
|---|---|---|---|---|
| Treating room like search | opening/scanning without categories | "The house ledger sorts mysteries into three questions." | "Work one category at a time: guest, object, room." | name first category mark |
| Suspect elimination stalls | rereading portraits with no physical action | "Portraits in this manor have backs for a reason." | "Put each alibi token in its own truth window and watch the mirrorline." | demonstrate one true alibi line |
| Object sideboard becomes prop soup | handling all objects randomly | "The tray is an inventory, not a pile." | "The sideboard is asking you to test objects, not inspect them forever." | place one wrong object on the plinth and show why it fails |
| Room stations are too abstract | staring at panels without using overlays | "The manor inspection window needs both proofs." | "Lay the suspect edge and object seal over each room under the work light." | demonstrate one false room alignment |
| Final accusation guessed | selector cover forcing, random slider changes, or one-player setting | "An accusation needs one proven answer from each station." | "Seat the proof, call the answer, have another player check it, then set it." | operator seats staff proof for proven categories and locks two selectors |

## Bottleneck check

- Required bottlenecks: final accusation only.
- Accidental bottlenecks: reading overload, one-player logic-grid domination, alibi-token swaps, object search soup, overlay glare/swap errors, selector-cover jams.
- Parallel work: suspect gallery and object sideboard can run in parallel after P1.
- Final convergence: suspect, object, and location selectors align in the mirrorline cabinet.

## Purpose mapping review

| Test | Pass? | Evidence | Required revision |
|---|---|---|---|
| Injection: distinct visible things have distinct understandable purposes | Draft pass | Ledger teaches categories; gallery produces suspect proof; sideboard produces object proof; room stations produce location proof; cabinet consumes all three. Guest bell is atmosphere only. | Keep visual language distinct for clue stations vs atmospheric props. |
| Surjection: every required purpose has a visible/documented home | Draft pass | Suspect, object, location, final accusation, reset, operator state, proof-locks, selector covers, and public proof tray now have homes. | Prototype proof-token sockets, proof tray, and selector covers at the cabinet. |
| Bijection: used elements make sense after use | Draft pass | Suspect crest is earned from a portrait truth-window contradiction; crest edge and object seal icon are reused at the room inspection windows, then all three proofs seat at the cabinet. | Prototype truth-window and overlay legibility/reset. |
| Intentional overlap/share: reused elements layer cleanly | Draft pass | Each proof token is earned at one station and reused at the cabinet; overlap is staged rather than hidden. | Ensure tokens do not reveal final answer before the team earns the category. |
| Build: earlier uses make later uses more meaningful | Draft pass | P1 teaches categories; P2/P3/P4 produce tangible proofs; proof tray makes progress public; P5 turns proofs into selector access and final accusation. | Make token placement visibly unlock each final category. |

| Visible element/zone | Purpose(s) | First use | Later reuse/build | Atmosphere or clue? | Risk |
|---|---|---|---|---|---|
| House ledger | onboarding, category model, hint frame | teaches suspect/object/location questions | operator can refer back during hints | clue | can become reading homework |
| Guest bell | mood, possible feedback | establishes living-house tone | maybe confirms transitions | atmosphere for now | audio-only clue confusion |
| Suspect gallery | suspect category, social texture | test alibi tokens in portrait-back truth windows | produces suspect proof token for cabinet; crest edge may help P4 | clue | token swaps, tiny windows, or text-heavy alibis |
| Object sideboard | object category, tactile work | test objects on mirrorline balance plinth | produces object proof token for cabinet; seal icon may help P4 | clue | prop loss, plinth tolerance, guessing by brute-force |
| Room stations | location category, spatial miniature | compare suspect/object overlays against room inspection windows | produces location proof token for cabinet | clue | glare, swapped overlays, fiddly reset |
| Proof tray / evidence rail | public progress, loose-part control | holds earned crest/seal/tile between stations and cabinet | gives operator and team a shared "ready for accusation" state | clue/ops | aisle crowding or prop dumping |
| Mirrorline cabinet | final convergence, state validation, signature moment | locked/teased early | accepts proof tokens, opens selector covers, then receives final accusation | clue/signature | cover forcing, socket guessing, one-player domination, false positive |
| House echo cards | optional delight, recall, fast-team reward | appear only after station proof is earned | final mirror echo ties all three solved categories together | atmosphere/reward | reading load or clue confusion |

## Promotion gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Aha | Draft pass | P2/P3/P4 now each have physical proof output and P5 consumes them | Test whether players feel deduction is physical. |
| Fairness | TBD | no simulation yet | Tune suspect/object/location count. |
| Theme | Draft pass | original spooky manor-in-a-trailer locked | Avoid protected board-game IP and generic logic worksheet feel. |
| Physicality | Draft | all beats have physical mechanisms; P2/P3/P4 rely on truth-window, behavior, and overlay tests; P5 uses proof-locked selector covers | Prototype mirrorline cabinet, portrait truth windows, and room inspection windows. |
| Purpose mapping | Draft pass | every required category now outputs a physical proof token with a cabinet home; guest bell is parked as atmosphere | Prototype proof-token sockets and verify P2/P3/P4 token earns are legible. |
| Economics | Draft | inventory-backed build draft | Price cabinet, portrait frames, props, and lighting. |
| Flow | Draft pass | spatial route now separates entry, evidence split, proof tray, and cabinet role lane | Verify with actual trailer dimensions and mockup crowding. |
| Ops | TBD | OPS draft only | Validate reset under 10 minutes. |
| Safety | TBD | SAFETY draft only | Resolve mirrors/acrylic, pinch, work-light glare, egress. |

