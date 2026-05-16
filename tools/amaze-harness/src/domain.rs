mod build;
mod catalog;
mod docs;
mod evidence;
mod ops;
mod playtest;
mod scenes;

use crate::markdown::{extract_table, tables_with_headers, Table};

pub(crate) use build::{Component, PartDiagram};
pub(crate) use catalog::{BeatPackage, InventoryItem};
pub(crate) use docs::{BuildDoc, CatalogDoc, OpsDoc, PlaytestDoc, ScenesDoc};
pub(crate) use evidence::EvidenceProbe;
pub(crate) use ops::{NpcCharacter, OperatorRotation};
pub(crate) use playtest::{ChaosProbe, VariabilityProfile};
pub(crate) use scenes::{
    BeatCard, BehaviorProbe, SceneSummary, TeamProbe, TransformationState, UnlockPath,
};

pub(crate) fn beat_cards_from_table(table: &Table) -> Vec<BeatCard> {
    table.rows.iter().map(BeatCard::from_row).collect()
}

pub(crate) fn beat_cards_from_markdown(markdown: &str) -> Option<Vec<BeatCard>> {
    extract_table(markdown, "## Beat cards").map(|table| beat_cards_from_table(&table))
}

pub(crate) fn scene_summaries_from_table(table: &Table) -> Vec<SceneSummary> {
    table.rows.iter().map(SceneSummary::from_row).collect()
}

pub(crate) fn scene_summaries_from_markdown(markdown: &str) -> Option<Vec<SceneSummary>> {
    extract_table(markdown, "## Scene list").map(|table| scene_summaries_from_table(&table))
}

pub(crate) fn components_from_table(table: &Table) -> Vec<Component> {
    table.rows.iter().map(Component::from_row).collect()
}

pub(crate) fn components_from_markdown(markdown: &str) -> Option<Vec<Component>> {
    extract_table(markdown, "## Bill of materials").map(|table| components_from_table(&table))
}

pub(crate) fn part_diagrams_from_table(table: &Table) -> Vec<PartDiagram> {
    table.rows.iter().map(PartDiagram::from_row).collect()
}

pub(crate) fn part_diagrams_from_markdown(markdown: &str) -> Option<Vec<PartDiagram>> {
    extract_table(markdown, "## Part diagrams").map(|table| part_diagrams_from_table(&table))
}

pub(crate) fn team_probes_from_table(table: &Table) -> Vec<TeamProbe> {
    table.rows.iter().map(TeamProbe::from_row).collect()
}

pub(crate) fn team_probes_from_markdown(markdown: &str) -> Option<Vec<TeamProbe>> {
    extract_table(markdown, "## Team archetype probes").map(|table| team_probes_from_table(&table))
}

pub(crate) fn behavior_probes_from_table(table: &Table) -> Vec<BehaviorProbe> {
    table.rows.iter().map(BehaviorProbe::from_row).collect()
}

pub(crate) fn behavior_probes_from_markdown(markdown: &str) -> Option<Vec<BehaviorProbe>> {
    extract_table(markdown, "## Behavior probes").map(|table| behavior_probes_from_table(&table))
}

pub(crate) fn transformation_states_from_table(table: &Table) -> Vec<TransformationState> {
    table
        .rows
        .iter()
        .map(TransformationState::from_row)
        .collect()
}

pub(crate) fn transformation_states_from_markdown(
    markdown: &str,
) -> Option<Vec<TransformationState>> {
    extract_table(markdown, "## Transformation states")
        .map(|table| transformation_states_from_table(&table))
}

pub(crate) fn unlock_paths_from_table(table: &Table) -> Vec<UnlockPath> {
    table.rows.iter().map(UnlockPath::from_row).collect()
}

pub(crate) fn unlock_paths_from_markdown(markdown: &str) -> Option<Vec<UnlockPath>> {
    extract_table(markdown, "## Unlock paths").map(|table| unlock_paths_from_table(&table))
}

pub(crate) fn npc_characters_from_table(table: &Table) -> Vec<NpcCharacter> {
    table.rows.iter().map(NpcCharacter::from_row).collect()
}

pub(crate) fn npc_characters_from_markdown(markdown: &str) -> Option<Vec<NpcCharacter>> {
    extract_table(markdown, "## NPC and operator characters")
        .map(|table| npc_characters_from_table(&table))
}

pub(crate) fn operator_rotations_from_table(table: &Table) -> Vec<OperatorRotation> {
    table.rows.iter().map(OperatorRotation::from_row).collect()
}

pub(crate) fn operator_rotations_from_markdown(markdown: &str) -> Option<Vec<OperatorRotation>> {
    extract_table(markdown, "## Multi-room operator rotation")
        .map(|table| operator_rotations_from_table(&table))
}

pub(crate) fn variability_profiles_from_table(table: &Table) -> Vec<VariabilityProfile> {
    table
        .rows
        .iter()
        .map(VariabilityProfile::from_row)
        .collect()
}

pub(crate) fn variability_profiles_from_markdown(
    markdown: &str,
) -> Option<Vec<VariabilityProfile>> {
    extract_table(markdown, "## Variability matrix")
        .map(|table| variability_profiles_from_table(&table))
}

pub(crate) fn chaos_probes_from_table(table: &Table) -> Vec<ChaosProbe> {
    table.rows.iter().map(ChaosProbe::from_row).collect()
}

pub(crate) fn chaos_probes_from_markdown(markdown: &str) -> Option<Vec<ChaosProbe>> {
    extract_table(markdown, "## Chaos protocol").map(|table| chaos_probes_from_table(&table))
}

pub(crate) fn bench_evidence_from_table(table: &Table) -> Vec<EvidenceProbe> {
    table
        .rows
        .iter()
        .map(EvidenceProbe::from_bench_row)
        .collect()
}

pub(crate) fn bench_evidence_from_markdown(markdown: &str) -> Vec<EvidenceProbe> {
    extract_table(markdown, "## Device bench tests")
        .map(|table| bench_evidence_from_table(&table))
        .unwrap_or_default()
}

pub(crate) fn admin_evidence_from_table(table: &Table) -> Vec<EvidenceProbe> {
    table
        .rows
        .iter()
        .map(EvidenceProbe::from_admin_row)
        .collect()
}

pub(crate) fn admin_evidence_from_markdown(markdown: &str) -> Vec<EvidenceProbe> {
    extract_table(markdown, "## Admin replacement drills")
        .map(|table| admin_evidence_from_table(&table))
        .unwrap_or_default()
}

pub(crate) fn chaos_evidence_from_table(table: &Table) -> Vec<EvidenceProbe> {
    table
        .rows
        .iter()
        .map(EvidenceProbe::from_chaos_row)
        .collect()
}

pub(crate) fn chaos_evidence_from_markdown(markdown: &str) -> Vec<EvidenceProbe> {
    extract_table(markdown, "## Chaos protocol")
        .map(|table| chaos_evidence_from_table(&table))
        .unwrap_or_default()
}

pub(crate) fn inventory_items_from_markdown(markdown: &str) -> Vec<InventoryItem> {
    tables_with_headers(markdown, &["ID", "Item"])
        .iter()
        .map(InventoryItem::from_row)
        .collect()
}

pub(crate) fn beat_packages_from_markdown(markdown: &str) -> Vec<BeatPackage> {
    tables_with_headers(markdown, &["Package ID", "Beat package"])
        .iter()
        .map(BeatPackage::from_row)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        admin_evidence_from_markdown, beat_cards_from_markdown, beat_packages_from_markdown,
        components_from_markdown, inventory_items_from_markdown, scene_summaries_from_markdown,
        BeatCard, BuildDoc, CatalogDoc, ChaosProbe, Component, EvidenceProbe, PartDiagram,
        PlaytestDoc, SceneSummary, ScenesDoc, TeamProbe, VariabilityProfile,
    };

    #[test]
    fn parses_beat_card_defaults() {
        let beat = BeatCard::from_row(&row(&[
            ("Beat", "P1 latch"),
            ("Target min", "3"),
            ("Reliability risk", "REL-JAM"),
        ]));

        assert_eq!(beat.name, "P1 latch");
        assert_eq!(beat.scene, "TBD");
        assert_eq!(beat.target_minutes, Some(3));
        assert_eq!(beat.target_or_default(), 3);
        assert_eq!(beat.slow_minutes, None);
        assert_eq!(beat.slow_or_default(), 5);
        assert_eq!(beat.hint_at_minutes, None);
        assert_eq!(beat.reliability_risk, "REL-JAM");
    }

    #[test]
    fn parses_scene_summary_rows() {
        let scene = SceneSummary::from_row(&row(&[
            ("Scene", "Dock"),
            ("Purpose", "onboard"),
            ("Clock", "5 min"),
            ("Team/behavior probes", "speed team"),
        ]));

        assert_eq!(scene.scene, "Dock");
        assert_eq!(scene.purpose, "onboard");
        assert_eq!(scene.clock, "5 min");
        assert_eq!(scene.team_behavior_probes, "speed team");
    }

    #[test]
    fn parses_component_optional_numeric_fields() {
        let component = Component::from_row(&row(&[
            ("Component", "hinge puzzle"),
            ("Inventory ID", "MECH-001"),
            ("Beat IDs", "P1"),
            ("Visual reference", "diagrams/hinge.excalidraw"),
            ("Build time", "4 hr"),
            ("Replacement min", "2"),
            ("Spare qty", "1"),
        ]));

        assert_eq!(component.name, "hinge puzzle");
        assert_eq!(component.visual_reference, "diagrams/hinge.excalidraw");
        assert_eq!(component.build_minutes, Some(4));
        assert_eq!(component.replacement_minutes, Some(2));
        assert_eq!(component.spare_qty, Some(1));
    }

    #[test]
    fn parses_part_diagram_rows() {
        let diagram = PartDiagram::from_row(&row(&[
            ("Beat/device", "P2 socket"),
            ("Diagram", "diagrams/socket.excalidraw"),
            ("What it proves", "reset path"),
            ("Missing before build readiness", "none"),
        ]));

        assert_eq!(diagram.scope, "P2 socket");
        assert_eq!(diagram.diagram, "diagrams/socket.excalidraw");
        assert_eq!(diagram.proof, "reset path");
        assert_eq!(diagram.build_readiness_gap, "none");
    }

    #[test]
    fn parses_simulation_probe_rows() {
        let team = TeamProbe::from_row(&row(&[
            ("Team archetype", "Speed team"),
            ("Scene/beat probe", "P2 rush"),
            ("Observable signal", "skips labels"),
        ]));
        let variability = VariabilityProfile::from_row(&row(&[
            ("Team/behavior", "Speed team"),
            ("Reliability issue IDs", "REL-FORCE"),
        ]));
        let chaos = ChaosProbe::from_row(&row(&[("Probe ID", "CHAOS-DUMP")]));

        assert_eq!(team.archetype, "Speed team");
        assert_eq!(team.scene_beat_probe, "P2 rush");
        assert_eq!(team.observable_signal, "skips labels");
        assert_eq!(variability.descriptor, "Speed team");
        assert_eq!(variability.issue_ids, "REL-FORCE");
        assert_eq!(chaos.probe_id, "CHAOS-DUMP");
    }

    #[test]
    fn parses_evidence_probe_rows() {
        let bench = EvidenceProbe::from_bench_row(&row(&[
            ("Test ID", "BT-1"),
            ("Device", "socket"),
            ("Risk band", "Orange"),
            ("Required observation", "no unsafe state"),
            ("Status", "Not run"),
            ("Promotion impact", "blocks P2"),
        ]));
        let admin = EvidenceProbe::from_admin_row(&row(&[
            ("Test ID", "AR-1"),
            ("Component", "switch"),
            ("Criticality", "C4"),
            ("Failure simulated", "switch fails"),
            ("Spare/bypass used", "manual card"),
            ("Target replacement min", "2"),
            ("Result", "Pending"),
            ("Required change", "stage card"),
        ]));

        assert_eq!(bench.kind, "bench");
        assert_eq!(bench.target, "socket");
        assert_eq!(admin.kind, "admin");
        assert_eq!(admin.evidence, "switch fails; manual card; target 2 min");
    }

    #[test]
    fn parses_catalog_rows() {
        let markdown = "\
# Catalog

| ID | Item | Common sources | Unit cost band | Durability | Typical use |
|---|---|---|---:|---|---|
| DEV-SOCKET-001 | socket | hardware | `$5-25` | B | proof |

| Package ID | Beat package | Use when | Techniques | Default devices | Evidence hooks |
|---|---|---|---|---|---|
| PKG-SOCKET-PROOF | Socket proof | keyed proof needed | `TECH-FIT-001` | `DEV-SOCKET-001` | bench socket fit |
";
        let inventory = inventory_items_from_markdown(markdown);
        let packages = beat_packages_from_markdown(markdown);

        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory[0].id, "DEV-SOCKET-001");
        assert!(inventory[0].matches_query("hardware"));
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].package_id, "PKG-SOCKET-PROOF");
        assert!(packages[0].matches_query("socket"));
    }

    #[test]
    fn catalog_doc_collects_inventory_and_packages() {
        let markdown = "\
# Catalog

| ID | Item | Common sources | Unit cost band | Durability | Typical use |
|---|---|---|---:|---|---|
| DEV-SOCKET-001 | socket | hardware | `$5-25` | B | proof |

| Package ID | Beat package | Use when | Techniques | Default devices | Evidence hooks |
|---|---|---|---|---|---|
| PKG-SOCKET-PROOF | Socket proof | keyed proof needed | `TECH-FIT-001` | `DEV-SOCKET-001` | bench socket fit |
";
        let catalog = CatalogDoc::from_markdown(markdown);

        assert_eq!(catalog.inventory_items[0].id, "DEV-SOCKET-001");
        assert_eq!(catalog.beat_packages[0].package_id, "PKG-SOCKET-PROOF");
    }

    #[test]
    fn named_loaders_read_expected_sections() {
        let scenes_markdown = "\
# Scenes

## Scene list

| Scene | Purpose | Clock | Team/behavior probes |
|---|---|---|---|
| Dock | onboarding | 5 min | speed team |

## Beat cards

| Beat | Scene | Target min | Reliability risk |
|---|---|---:|---|
| P1 latch | Dock | 3 | REL-JAM |
";
        let build = "\
# Build

## Bill of materials

| Component | Inventory ID | Beat IDs | Criticality | Visual reference |
|---|---|---|---|---|
| hinge puzzle | MECH-001 | P1 | C4 | diagrams/hinge.excalidraw |
";

        let scenes = scene_summaries_from_markdown(scenes_markdown).expect("scene list");
        let beats = beat_cards_from_markdown(scenes_markdown).expect("beat cards");
        let components = components_from_markdown(build).expect("bill of materials");

        assert_eq!(scenes[0].scene, "Dock");
        assert_eq!(beats[0].name, "P1 latch");
        assert_eq!(components[0].name, "hinge puzzle");
    }

    #[test]
    fn named_loaders_do_not_fall_back_to_similar_tables() {
        let markdown = "\
# Almost scenes

## Beat card backlog

| Beat | Scene | Target min |
|---|---|---:|
| P1 latch | Dock | 3 |

## Component wishlist

| Component | Inventory ID | Beat IDs |
|---|---|---|
| hinge puzzle | MECH-001 | P1 |
";

        assert!(beat_cards_from_markdown(markdown).is_none());
        assert!(components_from_markdown(markdown).is_none());
        assert!(scene_summaries_from_markdown(markdown).is_none());
    }

    #[test]
    fn evidence_loaders_default_missing_sections_to_empty() {
        let markdown = "# Playtest\n\nNo evidence tables yet.\n";

        assert!(admin_evidence_from_markdown(markdown).is_empty());
    }

    #[test]
    fn document_bundles_collect_related_sections() {
        let scenes_markdown = "\
# Scenes

## Scene list

| Scene | Purpose | Clock | Team/behavior probes |
|---|---|---|---|
| Dock | onboarding | 5 min | speed team |

## Beat cards

| Beat | Scene | Target min |
|---|---|---:|
| P1 latch | Dock | 3 |

## Team archetype probes

| Team archetype | Scene/beat probe | Observable signal |
|---|---|---|
| Amazing team | P1 | fast solve |
";
        let build_markdown = "\
# Build

## Bill of materials

| Component | Inventory ID | Beat IDs | Criticality |
|---|---|---|---|
| hinge puzzle | MECH-001 | P1 | C4 |

## Part diagrams

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P1 | diagrams/hinge.excalidraw | reset | none |
";
        let playtest_markdown = "\
# Playtest

## Variability matrix

| Team/behavior | Reliability issue IDs |
|---|---|
| Amazing team | REL-FAST |

## Chaos protocol

| Probe ID | Probe action |
|---|---|
| CHAOS-1 | misplace loose part |
";

        let scenes = ScenesDoc::from_markdown(scenes_markdown);
        let build = BuildDoc::from_markdown(build_markdown);
        let playtest = PlaytestDoc::from_markdown(playtest_markdown);

        assert_eq!(scenes.require_beat_cards().unwrap()[0].name, "P1 latch");
        assert_eq!(
            scenes.require_team_probes().unwrap()[0].archetype,
            "Amazing team"
        );
        assert_eq!(build.require_components().unwrap()[0].name, "hinge puzzle");
        assert_eq!(build.part_diagrams.as_ref().unwrap()[0].scope, "P1");
        assert_eq!(
            playtest.variability_profiles.as_ref().unwrap()[0].issue_ids,
            "REL-FAST"
        );
        assert_eq!(
            playtest.chaos_probes.as_ref().unwrap()[0].probe_id,
            "CHAOS-1"
        );
    }

    #[test]
    fn playtest_doc_collects_all_evidence_probe_kinds() {
        let markdown = "\
# Playtest

## Device bench tests

| Test ID | Device | Risk band | Required observation | Status | Promotion impact |
|---|---|---|---|---|---|
| BT-1 | switch | Orange | no unsafe state | pending | blocks P2 |

## Admin replacement drills

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Result | Required change |
|---|---|---|---|---|---:|---|---|
| AR-1 | switch | C4 | switch fails | manual card | 2 | Not run | stage card |

## Chaos protocol

| Probe ID | Beat/device | Probe action | Watch signal | Status |
|---|---|---|---|---|
| CH-1 | P2 | pull part | recovery starts | blocked |
";

        let kinds: Vec<_> = PlaytestDoc::from_markdown(markdown)
            .into_evidence_probes()
            .into_iter()
            .map(|probe| probe.kind)
            .collect();

        assert_eq!(kinds, vec!["bench", "admin", "chaos"]);
    }

    fn row(values: &[(&str, &str)]) -> HashMap<String, String> {
        values
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }
}
