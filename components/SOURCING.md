# Sourcing Standard

AMAZE should source parts from common, repeatable places before relying on
one-off props. Prices are planning bands until a purchase receipt or vendor quote
is recorded in the room build pack.

## Supplier tiers

| Tier | Examples | Best for | Risk |
|---|---|---|---|
| Big-box hardware | Home Depot, Lowe's, Ace | plywood, fasteners, hinges, casters, cable raceway, basic lighting | local stock varies |
| General retail | Amazon, Walmart, Target | commodity electronics, organizers, labels, batteries, cases | listings change and quality varies |
| Electronics hobby | Adafruit, SparkFun, Pololu | LEDs, sensors, microcontrollers, connectors, audio boards | shipping lead time, hobby durability |
| Electronics distributor | Digi-Key, Mouser | repeatable electronic parts, switches, connectors, power supplies | part selection requires care |
| Industrial supply | McMaster-Carr, Grainger, ULINE | robust hardware, latches, hinges, magnets, cases, labels | higher price, shipping cost |
| Office/print | Staples, FedEx Office, local print shop | laminated cards, labels, evidence tags, manuals | reprint color and stock variation |
| Thrift/scenic | thrift stores, surplus stores, salvage, Etsy | themed props, patina, unusual enclosures | not repeatable; use only for non-critical scenic dressing |
| Local fabrication | makerspace, cabinet shop, metal shop, 3D print | custom enclosures, brackets, durable panels | lead time and revision cost |

## Price confidence

| Confidence | Meaning | Promotion use |
|---|---|---|
| Planning | estimated from common retail memory or comparable parts | OK for seed/graphed rooms |
| Checked | price verified from a current listing or shelf check | required before prototype budget lock |
| Quoted | vendor/fabricator quote received | required for expensive custom pieces |
| Purchased | receipt exists | use as actual cost in playtest/build records |

Do not store private account details, personal contact info, or non-public vendor
pricing in the repo. Public supplier names and public price bands are OK.

## Cost band format

Use ranges instead of false precision:

| Band | Meaning |
|---|---|
| `$0-5` | consumable, printed part, small hardware |
| `$5-15` | small prop, switch, label set, simple organizer |
| `$15-40` | better hardware, audio player, LED kit, small enclosure |
| `$40-100` | rugged case, custom panel, quality latch set, tool |
| `$100-250` | central prop shell, fabricated module, robust electronics assembly |
| `$250+` | major scenic object, custom fabrication, high-duty enclosure |

## Substitution rule

Each inventory item should name an acceptable substitute class. If a component
cannot be sourced from at least two common paths, treat it as a custom build risk
and add a spare or bypass plan before promotion.

