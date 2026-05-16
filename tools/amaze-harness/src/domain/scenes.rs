use std::collections::HashMap;

use crate::markdown::{parse_minutes_field, value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BeatCard {
    pub(crate) name: String,
    pub(crate) scene: String,
    pub(crate) player_action: String,
    pub(crate) aha: String,
    pub(crate) check: String,
    pub(crate) target_minutes: Option<u32>,
    pub(crate) hint_at_minutes: Option<u32>,
    pub(crate) slow_minutes: Option<u32>,
    pub(crate) behavior_probe: String,
    pub(crate) mechanism: String,
    pub(crate) reliability_risk: String,
    pub(crate) dc: String,
    pub(crate) success: String,
    pub(crate) partial: String,
    pub(crate) stall_hint_trigger: String,
    pub(crate) reset_effect: String,
}

impl BeatCard {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        let target_minutes = parse_minutes_field(row, &["Target min", "Target", "Expected min"]);
        Self {
            name: value(row, "Beat"),
            scene: value(row, "Scene"),
            player_action: value(row, "Player action"),
            aha: value(row, "Aha"),
            check: value(row, "Check"),
            target_minutes,
            hint_at_minutes: parse_minutes_field(
                row,
                &["Hint at min", "Hint min", "Hint trigger min"],
            ),
            slow_minutes: parse_minutes_field(row, &["Slow max min", "Max min", "Slow min"]),
            behavior_probe: value(row, "Behavior probe"),
            mechanism: value(row, "Mechanism"),
            reliability_risk: value(row, "Reliability risk"),
            dc: value(row, "DC"),
            success: value(row, "Success"),
            partial: value(row, "Partial"),
            stall_hint_trigger: value(row, "Stall/hint trigger"),
            reset_effect: value(row, "Reset effect"),
        }
    }

    pub(crate) fn target_or_default(&self) -> u32 {
        self.target_minutes.unwrap_or(4)
    }

    pub(crate) fn slow_or_default(&self) -> u32 {
        self.slow_minutes
            .unwrap_or_else(|| self.target_or_default() + 2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SceneSummary {
    pub(crate) scene: String,
    pub(crate) purpose: String,
    pub(crate) clock: String,
    pub(crate) team_behavior_probes: String,
}

impl SceneSummary {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            scene: value(row, "Scene"),
            purpose: value(row, "Purpose"),
            clock: value(row, "Clock"),
            team_behavior_probes: value(row, "Team/behavior probes"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TeamProbe {
    pub(crate) archetype: String,
    pub(crate) scene_beat_probe: String,
    pub(crate) observable_signal: String,
}

impl TeamProbe {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            archetype: value(row, "Team archetype"),
            scene_beat_probe: value(row, "Scene/beat probe"),
            observable_signal: value(row, "Observable signal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BehaviorProbe {
    pub(crate) behavior: String,
    pub(crate) team_persona: String,
}

impl BehaviorProbe {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            behavior: value(row, "Behavior"),
            team_persona: value(row, "Team/persona"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TransformationState {
    pub(crate) id: String,
    pub(crate) beat: String,
    pub(crate) target: String,
    pub(crate) from_state: String,
    pub(crate) trigger: String,
    pub(crate) to_state: String,
    pub(crate) visible_proof: String,
    pub(crate) reset_state: String,
    pub(crate) failure_bypass: String,
}

impl TransformationState {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            id: value(row, "ID"),
            beat: value(row, "Beat"),
            target: value(row, "Object/space"),
            from_state: value(row, "From state"),
            trigger: value(row, "Trigger"),
            to_state: value(row, "To state"),
            visible_proof: value(row, "Visible proof"),
            reset_state: value(row, "Reset state"),
            failure_bypass: value(row, "Failure/bypass"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnlockPath {
    pub(crate) path: String,
    pub(crate) beats: String,
    pub(crate) unlocks: String,
    pub(crate) fast_coherence: String,
    pub(crate) slow_coherence: String,
    pub(crate) operator_acceleration: String,
}

impl UnlockPath {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            path: value(row, "Path"),
            beats: value(row, "Beats"),
            unlocks: value(row, "Unlocks"),
            fast_coherence: value(row, "Fast coherence"),
            slow_coherence: value(row, "Slow coherence"),
            operator_acceleration: value(row, "Operator acceleration"),
        }
    }
}
