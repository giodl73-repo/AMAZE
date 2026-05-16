use std::path::Path;

use crate::room::{
    read_optional_ops_doc, read_room_file, read_scenes_doc, room_contract_issues, room_file_path,
    RoomContractIssue, BUILD_FILE, OPS_FILE, SCENES_FILE,
};
use crate::rules::ops::check_ops_readiness;
use crate::rules::unlocks::check_unlock_readiness;
use crate::rules::visual::check_visual_references;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CheckFailure {
    RoomContract(RoomContractIssue),
    Unlock(String),
    Ops(String),
    Visual(String),
}

impl CheckFailure {
    fn message(&self) -> String {
        match self {
            Self::RoomContract(issue) => issue.message(),
            Self::Unlock(failure) => failure.clone(),
            Self::Ops(failure) => failure.clone(),
            Self::Visual(failure) => failure.clone(),
        }
    }
}

pub(crate) fn check_room(room: &Path) -> Result<(), String> {
    let (failures, mut notes) = check_failures(room)?;

    if failures.is_empty() {
        notes.sort();
        notes.dedup();
        for note in notes {
            println!("- {note}");
        }
        println!("room check passed: {}", room.display());
        Ok(())
    } else {
        for failure in failures {
            eprintln!("- {}", failure.message());
        }
        Err("room check failed".to_string())
    }
}

fn check_failures(room: &Path) -> Result<(Vec<CheckFailure>, Vec<String>), String> {
    let mut failures: Vec<CheckFailure> = room_contract_issues(room)?
        .into_iter()
        .map(CheckFailure::RoomContract)
        .collect();
    let mut visual_failures = Vec::new();
    let mut notes = Vec::new();

    let build_path = room_file_path(room, BUILD_FILE);
    if build_path.exists() {
        let build = read_room_file(room, BUILD_FILE)?;
        check_visual_references(room, &build, &mut visual_failures, &mut notes);
    }

    let scenes_path = room_file_path(room, SCENES_FILE);
    if scenes_path.exists() {
        let scenes_doc = read_scenes_doc(room)?;
        let beat_cards = scenes_doc.require_beat_cards()?;
        failures.extend(
            check_unlock_readiness(
                beat_cards,
                scenes_doc.transformation_states.as_deref(),
                scenes_doc.unlock_paths.as_deref(),
            )
            .into_iter()
            .map(CheckFailure::Unlock),
        );
    }

    let ops_path = room_file_path(room, OPS_FILE);
    if ops_path.exists() {
        let ops_doc = read_optional_ops_doc(room);
        failures.extend(
            check_ops_readiness(
                ops_doc.npc_characters.as_deref(),
                ops_doc.operator_rotations.as_deref(),
            )
            .into_iter()
            .map(CheckFailure::Ops),
        );
    }

    failures.extend(visual_failures.into_iter().map(CheckFailure::Visual));

    Ok((failures, notes))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{check_failures, CheckFailure};
    use crate::room::{RoomContractIssue, BUILD_FILE};

    #[test]
    fn check_keeps_room_contract_failures_structured_until_rendering() {
        let room = temp_room("structured-check");
        fs::create_dir_all(&room).expect("create temp room");

        let (failures, notes) = check_failures(&room).expect("check failures");

        assert!(notes.is_empty());
        assert!(failures.iter().any(|failure| matches!(
            failure,
            CheckFailure::RoomContract(RoomContractIssue::MissingFile { path })
                if path.ends_with(BUILD_FILE)
        )));
        assert!(failures
            .iter()
            .any(|failure| failure.message().contains("missing required room file")));

        cleanup(room);
    }

    fn temp_room(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("amaze-check-test-{label}-{unique}"))
    }

    fn cleanup(path: PathBuf) {
        let _ = fs::remove_dir_all(path);
    }
}
