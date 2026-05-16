use crate::domain::{NpcCharacter, OperatorRotation};
use crate::rules::unlocks::is_tbd;

pub(crate) fn check_ops_readiness(
    npcs: Option<&[NpcCharacter]>,
    rotations: Option<&[OperatorRotation]>,
) -> Vec<String> {
    let mut failures = Vec::new();

    if let Some(npcs) = npcs {
        for npc in npcs {
            failures.extend(check_npc(npc));
        }
    }

    if let Some(rotations) = rotations {
        for rotation in rotations {
            failures.extend(check_rotation(rotation));
        }
    }

    failures
}

fn check_npc(npc: &NpcCharacter) -> Vec<String> {
    let label = if is_tbd(&npc.name) {
        "<unnamed>".to_string()
    } else {
        npc.name.clone()
    };
    let mut failures = Vec::new();

    for (field, value) in [
        ("NPC/voice", npc.name.as_str()),
        ("Function", npc.function.as_str()),
        ("Trigger", npc.trigger.as_str()),
        ("Delivery mode", npc.delivery_mode.as_str()),
        ("Sample line", npc.sample_line.as_str()),
        ("Limits", npc.limits.as_str()),
        ("Reset/ops burden", npc.reset_ops_burden.as_str()),
    ] {
        if is_tbd(value) {
            failures.push(format!("NPC/operator character {label} missing {field}"));
        }
    }

    failures
}

fn check_rotation(rotation: &OperatorRotation) -> Vec<String> {
    let label = if is_tbd(&rotation.scope) {
        "<unnamed>".to_string()
    } else {
        rotation.scope.clone()
    };
    let mut failures = Vec::new();

    for (field, value) in [
        ("Scope", rotation.scope.as_str()),
        ("Coverage model", rotation.coverage_model.as_str()),
        ("Max rooms", rotation.max_rooms.as_str()),
        ("Safe when", rotation.safe_when.as_str()),
        ("Unsafe when", rotation.unsafe_when.as_str()),
        ("Handoff signal", rotation.handoff_signal.as_str()),
        (
            "Dedicated staff trigger",
            rotation.dedicated_staff_trigger.as_str(),
        ),
    ] {
        if is_tbd(value) {
            failures.push(format!("operator rotation {label} missing {field}"));
        }
    }

    if rotation.max_rooms.parse::<u32>().is_err() {
        failures.push(format!(
            "operator rotation {label} Max rooms must be a number"
        ));
    }

    failures
}

#[cfg(test)]
mod tests {
    use super::check_ops_readiness;
    use crate::domain::{NpcCharacter, OperatorRotation};

    #[test]
    fn flags_incomplete_npc_and_rotation_rows() {
        let npcs = vec![NpcCharacter {
            name: "Archivist".to_string(),
            function: "hint".to_string(),
            trigger: String::new(),
            delivery_mode: "radio".to_string(),
            sample_line: "The archive remembers.".to_string(),
            limits: "no answer".to_string(),
            reset_ops_burden: "audio cue".to_string(),
        }];
        let rotations = vec![OperatorRotation {
            scope: "Trailer cluster".to_string(),
            coverage_model: "roving operator".to_string(),
            max_rooms: "two".to_string(),
            safe_when: "camera wall live".to_string(),
            unsafe_when: "active reset".to_string(),
            handoff_signal: "status light".to_string(),
            dedicated_staff_trigger: "comfort pause".to_string(),
        }];

        let failures = check_ops_readiness(Some(&npcs), Some(&rotations));

        assert!(failures.iter().any(|failure| failure.contains("Trigger")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("Max rooms must be a number")));
    }
}
