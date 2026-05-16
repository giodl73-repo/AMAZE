use super::{
    admin_evidence_from_markdown, beat_cards_from_markdown, beat_packages_from_markdown,
    behavior_probes_from_markdown, bench_evidence_from_markdown, chaos_evidence_from_markdown,
    chaos_probes_from_markdown, components_from_markdown, inventory_items_from_markdown,
    npc_characters_from_markdown, operator_rotations_from_markdown, part_diagrams_from_markdown,
    scene_summaries_from_markdown, team_probes_from_markdown, transformation_states_from_markdown,
    unlock_paths_from_markdown, variability_profiles_from_markdown, BeatCard, BeatPackage,
    BehaviorProbe, ChaosProbe, Component, EvidenceProbe, InventoryItem, NpcCharacter,
    OperatorRotation, PartDiagram, SceneSummary, TeamProbe, TransformationState, UnlockPath,
    VariabilityProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScenesDoc {
    pub(crate) scene_summaries: Option<Vec<SceneSummary>>,
    pub(crate) beat_cards: Option<Vec<BeatCard>>,
    pub(crate) team_probes: Option<Vec<TeamProbe>>,
    pub(crate) behavior_probes: Option<Vec<BehaviorProbe>>,
    pub(crate) transformation_states: Option<Vec<TransformationState>>,
    pub(crate) unlock_paths: Option<Vec<UnlockPath>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OpsDoc {
    pub(crate) npc_characters: Option<Vec<NpcCharacter>>,
    pub(crate) operator_rotations: Option<Vec<OperatorRotation>>,
}

impl OpsDoc {
    pub(crate) fn from_markdown(markdown: &str) -> Self {
        Self {
            npc_characters: npc_characters_from_markdown(markdown),
            operator_rotations: operator_rotations_from_markdown(markdown),
        }
    }
}

impl ScenesDoc {
    pub(crate) fn from_markdown(markdown: &str) -> Self {
        Self {
            scene_summaries: scene_summaries_from_markdown(markdown),
            beat_cards: beat_cards_from_markdown(markdown),
            team_probes: team_probes_from_markdown(markdown),
            behavior_probes: behavior_probes_from_markdown(markdown),
            transformation_states: transformation_states_from_markdown(markdown),
            unlock_paths: unlock_paths_from_markdown(markdown),
        }
    }

    pub(crate) fn require_beat_cards(&self) -> Result<&[BeatCard], String> {
        self.beat_cards
            .as_deref()
            .ok_or_else(|| "SCENES.md does not contain a Beat cards table".to_string())
    }

    pub(crate) fn require_team_probes(&self) -> Result<&[TeamProbe], String> {
        self.team_probes
            .as_deref()
            .ok_or_else(|| "SCENES.md does not contain a Team archetype probes table".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BuildDoc {
    pub(crate) components: Option<Vec<Component>>,
    pub(crate) part_diagrams: Option<Vec<PartDiagram>>,
}

impl BuildDoc {
    pub(crate) fn from_markdown(markdown: &str) -> Self {
        Self {
            components: components_from_markdown(markdown),
            part_diagrams: part_diagrams_from_markdown(markdown),
        }
    }

    pub(crate) fn require_components(&self) -> Result<&[Component], String> {
        self.components
            .as_deref()
            .ok_or_else(|| "BUILD.md does not contain a Bill of materials table".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlaytestDoc {
    pub(crate) variability_profiles: Option<Vec<VariabilityProfile>>,
    pub(crate) chaos_probes: Option<Vec<ChaosProbe>>,
    pub(crate) bench_evidence: Vec<EvidenceProbe>,
    pub(crate) admin_evidence: Vec<EvidenceProbe>,
    pub(crate) chaos_evidence: Vec<EvidenceProbe>,
}

impl PlaytestDoc {
    pub(crate) fn from_markdown(markdown: &str) -> Self {
        Self {
            variability_profiles: variability_profiles_from_markdown(markdown),
            chaos_probes: chaos_probes_from_markdown(markdown),
            bench_evidence: bench_evidence_from_markdown(markdown),
            admin_evidence: admin_evidence_from_markdown(markdown),
            chaos_evidence: chaos_evidence_from_markdown(markdown),
        }
    }

    pub(crate) fn into_evidence_probes(self) -> Vec<EvidenceProbe> {
        let mut probes = self.bench_evidence;
        probes.extend(self.admin_evidence);
        probes.extend(self.chaos_evidence);
        probes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogDoc {
    pub(crate) inventory_items: Vec<InventoryItem>,
    pub(crate) beat_packages: Vec<BeatPackage>,
}

impl CatalogDoc {
    pub(crate) fn from_markdown(markdown: &str) -> Self {
        Self {
            inventory_items: inventory_items_from_markdown(markdown),
            beat_packages: beat_packages_from_markdown(markdown),
        }
    }
}
