use crate::domain::{BeatCard, TransformationState, UnlockPath};

pub(crate) fn check_unlock_readiness(
    beat_cards: &[BeatCard],
    transformations: Option<&[TransformationState]>,
    unlock_paths: Option<&[UnlockPath]>,
) -> Vec<String> {
    let mut failures = Vec::new();

    if let Some(transformations) = transformations {
        for transformation in transformations {
            failures.extend(check_transformation(beat_cards, transformation));
        }
    }

    if let Some(unlock_paths) = unlock_paths {
        for unlock_path in unlock_paths {
            failures.extend(check_unlock_path(beat_cards, unlock_path));
        }
    }

    failures
}

pub(crate) fn beat_id(beat_name: &str) -> Option<String> {
    beat_name
        .split_whitespace()
        .next()
        .filter(|token| token.starts_with('P') && token[1..].chars().all(|ch| ch.is_ascii_digit()))
        .map(str::to_string)
}

pub(crate) fn unlock_beat_ids(raw: &str) -> Vec<String> {
    let mut ids = Vec::new();

    for token in raw.split(|ch: char| ch == ',' || ch == '/' || ch.is_whitespace()) {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((start, end)) = trimmed.split_once('-') {
            if let (Some(start_num), Some(end_num)) = (beat_number(start), beat_number(end)) {
                for number in start_num..=end_num {
                    ids.push(format!("P{number}"));
                }
                continue;
            }
        }

        if beat_number(trimmed).is_some() {
            ids.push(trimmed.to_string());
        }
    }

    ids.sort();
    ids.dedup();
    ids
}

pub(crate) fn is_tbd(value: &str) -> bool {
    let normalized = value.trim().to_lowercase();
    normalized.is_empty() || matches!(normalized.as_str(), "tbd" | "none" | "n/a" | "na")
}

fn check_transformation(
    beat_cards: &[BeatCard],
    transformation: &TransformationState,
) -> Vec<String> {
    let label = transformation_label(transformation);
    let mut failures = Vec::new();

    for (field, value) in [
        ("ID", transformation.id.as_str()),
        ("Beat", transformation.beat.as_str()),
        ("Object/space", transformation.target.as_str()),
        ("From state", transformation.from_state.as_str()),
        ("Trigger", transformation.trigger.as_str()),
        ("To state", transformation.to_state.as_str()),
        ("Visible proof", transformation.visible_proof.as_str()),
        ("Reset state", transformation.reset_state.as_str()),
        ("Failure/bypass", transformation.failure_bypass.as_str()),
    ] {
        if is_tbd(value) {
            failures.push(format!("transformation {label} missing {field}"));
        }
    }

    let beat_ids = unlock_beat_ids(&transformation.beat);
    if beat_ids.is_empty() {
        failures.push(format!(
            "transformation {label} Beat must reference at least one beat ID such as P3"
        ));
    } else {
        failures.extend(missing_beat_failures(
            beat_cards,
            &beat_ids,
            &format!("transformation {label}"),
        ));
    }

    failures
}

fn check_unlock_path(beat_cards: &[BeatCard], unlock_path: &UnlockPath) -> Vec<String> {
    let label = if is_tbd(&unlock_path.path) {
        "<unnamed>".to_string()
    } else {
        unlock_path.path.clone()
    };
    let mut failures = Vec::new();

    for (field, value) in [
        ("Path", unlock_path.path.as_str()),
        ("Beats", unlock_path.beats.as_str()),
        ("Unlocks", unlock_path.unlocks.as_str()),
        ("Fast coherence", unlock_path.fast_coherence.as_str()),
        ("Slow coherence", unlock_path.slow_coherence.as_str()),
        (
            "Operator acceleration",
            unlock_path.operator_acceleration.as_str(),
        ),
    ] {
        if is_tbd(value) {
            failures.push(format!("unlock path {label} missing {field}"));
        }
    }

    let beat_ids = unlock_beat_ids(&unlock_path.beats);
    if beat_ids.is_empty() {
        failures.push(format!(
            "unlock path {label} Beats must reference at least one beat ID such as P2-P4"
        ));
    } else {
        failures.extend(missing_beat_failures(
            beat_cards,
            &beat_ids,
            &format!("unlock path {label}"),
        ));
    }

    failures
}

fn missing_beat_failures(beat_cards: &[BeatCard], refs: &[String], label: &str) -> Vec<String> {
    refs.iter()
        .filter(|id| {
            !beat_cards
                .iter()
                .any(|beat| beat_id(&beat.name).is_some_and(|candidate| candidate == **id))
        })
        .map(|id| format!("{label} references unknown beat {id}"))
        .collect()
}

fn transformation_label(transformation: &TransformationState) -> String {
    if !is_tbd(&transformation.id) {
        transformation.id.clone()
    } else if !is_tbd(&transformation.target) {
        transformation.target.clone()
    } else {
        "<unnamed>".to_string()
    }
}

fn beat_number(value: &str) -> Option<u32> {
    value.strip_prefix('P')?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{beat_id, check_unlock_readiness, unlock_beat_ids};
    use crate::domain::{BeatCard, TransformationState, UnlockPath};

    #[test]
    fn extracts_unlock_beat_ranges_and_lists() {
        assert_eq!(
            unlock_beat_ids("P2-P4, P7/P8"),
            vec![
                "P2".to_string(),
                "P3".to_string(),
                "P4".to_string(),
                "P7".to_string(),
                "P8".to_string()
            ]
        );
    }

    #[test]
    fn extracts_leading_beat_id() {
        assert_eq!(beat_id("P10 Dawn coordinate"), Some("P10".to_string()));
        assert_eq!(beat_id("Dawn coordinate"), None);
    }

    #[test]
    fn flags_missing_transformation_and_unlock_evidence() {
        let beats = vec![beat("P1 Intake")];
        let transformations = vec![TransformationState {
            id: "T1".to_string(),
            beat: "P2".to_string(),
            target: "cabinet".to_string(),
            from_state: "closed".to_string(),
            trigger: "P1 complete".to_string(),
            to_state: "open".to_string(),
            visible_proof: "".to_string(),
            reset_state: "closed".to_string(),
            failure_bypass: "staff key".to_string(),
        }];
        let paths = vec![UnlockPath {
            path: "Act I".to_string(),
            beats: "P1-P2".to_string(),
            unlocks: "cabinet".to_string(),
            fast_coherence: "earned".to_string(),
            slow_coherence: "recapped".to_string(),
            operator_acceleration: "TBD".to_string(),
        }];

        let failures = check_unlock_readiness(&beats, Some(&transformations), Some(&paths));

        assert!(failures
            .iter()
            .any(|failure| failure.contains("Visible proof")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("unknown beat P2")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("Operator acceleration")));
    }

    fn beat(name: &str) -> BeatCard {
        BeatCard {
            name: name.to_string(),
            scene: "S1".to_string(),
            player_action: String::new(),
            aha: String::new(),
            check: String::new(),
            target_minutes: Some(1),
            hint_at_minutes: Some(1),
            slow_minutes: Some(2),
            behavior_probe: String::new(),
            mechanism: String::new(),
            reliability_risk: String::new(),
            dc: String::new(),
            success: String::new(),
            partial: String::new(),
            stall_hint_trigger: String::new(),
            reset_effect: String::new(),
        }
    }
}
