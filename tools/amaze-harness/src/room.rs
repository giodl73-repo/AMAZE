use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::{BuildDoc, OpsDoc, PlaytestDoc, ScenesDoc};

pub(crate) const SCENES_FILE: &str = "SCENES.md";
pub(crate) const BUILD_FILE: &str = "BUILD.md";
pub(crate) const PLAYTEST_FILE: &str = "PLAYTEST.md";
pub(crate) const BRIEF_FILE: &str = "BRIEF.md";
pub(crate) const FLOORPLAN_FILE: &str = "FLOORPLAN.md";
pub(crate) const PUZZLE_GRAPH_FILE: &str = "PUZZLE-GRAPH.md";
pub(crate) const SAFETY_FILE: &str = "SAFETY.md";
pub(crate) const OPS_FILE: &str = "OPS.md";

pub(crate) const REQUIRED_ROOM_FILES: &[&str] = &[
    BRIEF_FILE,
    FLOORPLAN_FILE,
    PUZZLE_GRAPH_FILE,
    SCENES_FILE,
    BUILD_FILE,
    SAFETY_FILE,
    PLAYTEST_FILE,
    OPS_FILE,
];

pub(crate) const REQUIRED_BRIEF_HEADINGS: &[&str] = &[
    "## Status",
    "## Concept",
    "## Invariants check",
    "## Success criteria",
];

pub(crate) const REQUIRED_FLOORPLAN_HEADINGS: &[&str] = &[
    "## Envelope",
    "## Zone map",
    "## Flow",
    "## Egress and safety paths",
    "## Operator sightlines",
    "## Accessibility and reach",
    "## Reset paths",
    "## Transport and mounting notes",
    "## Open spatial risks",
];

pub(crate) const REQUIRED_PUZZLE_GRAPH_HEADINGS: &[&str] =
    &["## Nodes", "## Technique play profile", "## Edges"];

pub(crate) const REQUIRED_SCENES_HEADINGS: &[&str] = &[
    "## Party assumptions",
    "## Team archetype probes",
    "## Behavior probes",
    "## Scene list",
    "## Beat cards",
    "## Operator interventions",
];

pub(crate) const REQUIRED_PLAYTEST_HEADINGS: &[&str] = &[
    "## Simulation and playtest runs",
    "## Team archetype evidence",
    "## Behavior evidence",
    "## Stuck-state log",
    "## Scores",
];

pub(crate) const REQUIRED_BUILD_HEADINGS: &[&str] = &[
    "## Budget summary",
    "## Puzzle mechanism map",
    "## Bill of materials",
    "## Reliability risks",
    "## Device review matrix",
    "## Part diagrams",
    "## Spare kit",
    "## Build and replacement schedule",
    "## Criticality map",
    "## Build readiness gates",
];

pub(crate) const REQUIRED_SAFETY_HEADINGS: &[&str] = &[
    "## Egress",
    "## Hazards",
    "## Accessibility",
    "## Operator controls",
    "## Safety gate",
];

pub(crate) const REQUIRED_OPS_HEADINGS: &[&str] = &[
    "## Staffing",
    "## Run clock",
    "## Hint protocol",
    "## Reset checklist",
    "## Failure modes",
    "## Transport and maintenance",
    "## Operator stress tests",
];

pub(crate) const REQUIRED_HEADING_SETS: &[(&str, &[&str])] = &[
    (BRIEF_FILE, REQUIRED_BRIEF_HEADINGS),
    (FLOORPLAN_FILE, REQUIRED_FLOORPLAN_HEADINGS),
    (PUZZLE_GRAPH_FILE, REQUIRED_PUZZLE_GRAPH_HEADINGS),
    (SCENES_FILE, REQUIRED_SCENES_HEADINGS),
    (BUILD_FILE, REQUIRED_BUILD_HEADINGS),
    (SAFETY_FILE, REQUIRED_SAFETY_HEADINGS),
    (PLAYTEST_FILE, REQUIRED_PLAYTEST_HEADINGS),
    (OPS_FILE, REQUIRED_OPS_HEADINGS),
];

pub(crate) fn active_room_paths(rooms_root: &Path) -> Result<Vec<PathBuf>, String> {
    let entries = fs::read_dir(rooms_root)
        .map_err(|err| format!("failed to read {}: {err}", rooms_root.display()))?;
    let mut rooms = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read room entry: {err}"))?;
        let path = entry.path();
        if !path.is_dir() || !room_file_path(&path, BUILD_FILE).exists() {
            continue;
        }
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("TEMPLATE"))
        {
            continue;
        }
        rooms.push(path);
    }

    rooms.sort();
    Ok(rooms)
}

pub(crate) fn room_file_path(room: &Path, file_name: &str) -> PathBuf {
    room.join(file_name)
}

pub(crate) fn read_room_file(room: &Path, file_name: &str) -> Result<String, String> {
    let path = room_file_path(room, file_name);
    fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))
}

pub(crate) fn write_room_file(room: &Path, file_name: &str, contents: &str) -> Result<(), String> {
    let path = room_file_path(room, file_name);
    fs::write(&path, contents).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RoomContractIssue {
    MissingFile { path: PathBuf },
    MissingHeading { file_name: String, heading: String },
}

impl RoomContractIssue {
    pub(crate) fn message(&self) -> String {
        match self {
            Self::MissingFile { path } => {
                format!("missing required room file: {}", path.display())
            }
            Self::MissingHeading { file_name, heading } => {
                format!("{file_name} missing heading: {heading}")
            }
        }
    }
}

pub(crate) fn required_file_issues(room: &Path) -> Vec<RoomContractIssue> {
    REQUIRED_ROOM_FILES
        .iter()
        .map(|file| room_file_path(room, file))
        .filter(|path| !path.exists())
        .map(|path| RoomContractIssue::MissingFile { path })
        .collect()
}

#[cfg(test)]
pub(crate) fn required_file_failures(room: &Path) -> Vec<String> {
    required_file_issues(room)
        .iter()
        .map(RoomContractIssue::message)
        .collect()
}

pub(crate) fn room_contract_issues(room: &Path) -> Result<Vec<RoomContractIssue>, String> {
    let mut issues = required_file_issues(room);

    for (file_name, headings) in REQUIRED_HEADING_SETS {
        if room_file_path(room, file_name).exists() {
            let markdown = read_room_file(room, file_name)?;
            issues.extend(required_heading_issues(file_name, &markdown, headings));
        }
    }

    Ok(issues)
}

#[cfg(test)]
pub(crate) fn room_contract_failures(room: &Path) -> Result<Vec<String>, String> {
    Ok(room_contract_issues(room)?
        .iter()
        .map(RoomContractIssue::message)
        .collect())
}

pub(crate) fn required_heading_issues(
    file_name: &str,
    markdown: &str,
    required_headings: &[&str],
) -> Vec<RoomContractIssue> {
    required_headings
        .iter()
        .filter(|heading| !has_heading_line(markdown, heading))
        .map(|heading| RoomContractIssue::MissingHeading {
            file_name: file_name.to_string(),
            heading: (*heading).to_string(),
        })
        .collect()
}

fn has_heading_line(markdown: &str, required_heading: &str) -> bool {
    markdown.lines().any(|line| line.trim() == required_heading)
}

#[cfg(test)]
pub(crate) fn required_heading_failures(
    file_name: &str,
    markdown: &str,
    required_headings: &[&str],
) -> Vec<String> {
    required_heading_issues(file_name, markdown, required_headings)
        .iter()
        .map(RoomContractIssue::message)
        .collect()
}

pub(crate) fn read_optional_room_file(room: &Path, file_name: &str) -> String {
    read_room_file(room, file_name).unwrap_or_default()
}

pub(crate) fn read_scenes_doc(room: &Path) -> Result<ScenesDoc, String> {
    read_room_file(room, SCENES_FILE).map(|markdown| ScenesDoc::from_markdown(&markdown))
}

pub(crate) fn read_build_doc(room: &Path) -> Result<BuildDoc, String> {
    read_room_file(room, BUILD_FILE).map(|markdown| BuildDoc::from_markdown(&markdown))
}

pub(crate) fn read_optional_build_doc(room: &Path) -> BuildDoc {
    BuildDoc::from_markdown(&read_optional_room_file(room, BUILD_FILE))
}

pub(crate) fn read_playtest_doc(room: &Path) -> Result<PlaytestDoc, String> {
    read_room_file(room, PLAYTEST_FILE).map(|markdown| PlaytestDoc::from_markdown(&markdown))
}

pub(crate) fn read_optional_playtest_doc(room: &Path) -> PlaytestDoc {
    PlaytestDoc::from_markdown(&read_optional_room_file(room, PLAYTEST_FILE))
}

pub(crate) fn read_optional_ops_doc(room: &Path) -> OpsDoc {
    OpsDoc::from_markdown(&read_optional_room_file(room, OPS_FILE))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        active_room_paths, read_build_doc, read_optional_build_doc, read_optional_playtest_doc,
        read_room_file, read_scenes_doc, required_file_failures, required_file_issues,
        required_heading_failures, required_heading_issues, room_contract_failures,
        room_contract_issues, write_room_file, RoomContractIssue, BRIEF_FILE, BUILD_FILE,
        FLOORPLAN_FILE, OPS_FILE, PLAYTEST_FILE, PUZZLE_GRAPH_FILE, REQUIRED_BRIEF_HEADINGS,
        REQUIRED_BUILD_HEADINGS, REQUIRED_FLOORPLAN_HEADINGS, REQUIRED_OPS_HEADINGS,
        REQUIRED_PUZZLE_GRAPH_HEADINGS, REQUIRED_SAFETY_HEADINGS, SAFETY_FILE, SCENES_FILE,
    };

    #[test]
    fn reads_required_room_files_with_path_errors() {
        let room = temp_room("required-read");
        fs::create_dir_all(&room).expect("create temp room");
        write_room_file(&room, SCENES_FILE, "# Scenes\n").expect("write scenes");

        assert_eq!(read_room_file(&room, SCENES_FILE).unwrap(), "# Scenes\n");
        assert!(read_room_file(&room, BUILD_FILE)
            .unwrap_err()
            .contains("BUILD.md"));

        cleanup(room);
    }

    #[test]
    fn writes_room_files_with_path_errors() {
        let room = temp_room("write-room-file");
        fs::create_dir_all(&room).expect("create temp room");

        write_room_file(&room, BUILD_FILE, "# Build\n").expect("write build");

        assert_eq!(read_room_file(&room, BUILD_FILE).unwrap(), "# Build\n");
        assert!(
            write_room_file(&room.join("missing"), BUILD_FILE, "# Build\n")
                .unwrap_err()
                .contains("BUILD.md")
        );

        cleanup(room);
    }

    #[test]
    fn reports_missing_required_headings_with_file_context() {
        let issues = required_heading_issues(
            SCENES_FILE,
            "# Scenes\n\n## Beat cards\n",
            &["## Beat cards", "## Operator interventions"],
        );
        let failures = required_heading_failures(
            SCENES_FILE,
            "# Scenes\n\n## Beat cards\n",
            &["## Beat cards", "## Operator interventions"],
        );

        assert_eq!(
            issues,
            vec![RoomContractIssue::MissingHeading {
                file_name: SCENES_FILE.to_string(),
                heading: "## Operator interventions".to_string()
            }]
        );
        assert_eq!(
            failures,
            vec!["SCENES.md missing heading: ## Operator interventions"]
        );
    }

    #[test]
    fn required_headings_must_be_actual_heading_lines() {
        let issues = required_heading_issues(
            SCENES_FILE,
            "\
# Scenes

| Note |
|---|
| TODO: add ## Operator interventions |

The template says ## Operator interventions belongs here.
",
            &["## Operator interventions"],
        );

        assert_eq!(
            issues,
            vec![RoomContractIssue::MissingHeading {
                file_name: SCENES_FILE.to_string(),
                heading: "## Operator interventions".to_string()
            }]
        );
    }

    #[test]
    fn reports_missing_required_room_files_with_paths() {
        let room = temp_room("required-file-contract");
        fs::create_dir_all(&room).expect("create temp room");
        for file in [
            BRIEF_FILE,
            FLOORPLAN_FILE,
            PUZZLE_GRAPH_FILE,
            SCENES_FILE,
            SAFETY_FILE,
            PLAYTEST_FILE,
            OPS_FILE,
        ] {
            write_room_file(&room, file, "# Present\n").expect("write required file");
        }

        let issues = required_file_issues(&room);
        let failures = required_file_failures(&room);

        assert!(matches!(issues[0], RoomContractIssue::MissingFile { .. }));
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("BUILD.md"));

        cleanup(room);
    }

    #[test]
    fn room_contract_combines_required_file_and_heading_failures() {
        let room = temp_room("room-contract");
        fs::create_dir_all(&room).expect("create temp room");
        for file in [
            BRIEF_FILE,
            FLOORPLAN_FILE,
            PUZZLE_GRAPH_FILE,
            BUILD_FILE,
            SAFETY_FILE,
            OPS_FILE,
        ] {
            write_room_file(&room, file, "# Present\n").expect("write required file");
        }
        write_room_file(&room, SCENES_FILE, "# Scenes\n\n## Beat cards\n").expect("write scenes");

        let issues = room_contract_issues(&room).expect("room contract");
        let failures = room_contract_failures(&room).expect("room contract");

        assert!(issues
            .iter()
            .any(|issue| matches!(issue, RoomContractIssue::MissingFile { path } if path.ends_with(PLAYTEST_FILE))));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == SCENES_FILE && heading == "## Operator interventions"
        )));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("PLAYTEST.md")));
        assert!(failures
            .iter()
            .any(|failure| failure == "SCENES.md missing heading: ## Operator interventions"));

        cleanup(room);
    }

    #[test]
    fn room_contract_checks_build_headings() {
        let room = temp_room("build-headings");
        fs::create_dir_all(&room).expect("create temp room");
        for file in [
            BRIEF_FILE,
            FLOORPLAN_FILE,
            PUZZLE_GRAPH_FILE,
            SCENES_FILE,
            SAFETY_FILE,
            PLAYTEST_FILE,
            OPS_FILE,
        ] {
            write_room_file(&room, file, "# Present\n").expect("write required file");
        }
        write_room_file(&room, BUILD_FILE, "# Build\n\n## Bill of materials\n")
            .expect("write build");

        let issues = room_contract_issues(&room).expect("room contract");

        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == BUILD_FILE && heading == REQUIRED_BUILD_HEADINGS[0]
        )));

        cleanup(room);
    }

    #[test]
    fn room_contract_checks_safety_and_ops_headings() {
        let room = temp_room("safety-ops-headings");
        fs::create_dir_all(&room).expect("create temp room");
        for file in [
            BRIEF_FILE,
            FLOORPLAN_FILE,
            PUZZLE_GRAPH_FILE,
            SCENES_FILE,
            BUILD_FILE,
            PLAYTEST_FILE,
        ] {
            write_room_file(&room, file, "# Present\n").expect("write required file");
        }
        write_room_file(&room, SAFETY_FILE, "# Safety\n\n## Egress\n").expect("write safety");
        write_room_file(&room, OPS_FILE, "# Ops\n\n## Staffing\n").expect("write ops");

        let issues = room_contract_issues(&room).expect("room contract");

        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == SAFETY_FILE && heading == REQUIRED_SAFETY_HEADINGS[1]
        )));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == OPS_FILE && heading == REQUIRED_OPS_HEADINGS[1]
        )));

        cleanup(room);
    }

    #[test]
    fn room_contract_checks_brief_floorplan_and_puzzle_graph_headings() {
        let room = temp_room("remaining-headings");
        fs::create_dir_all(&room).expect("create temp room");
        for file in [
            SCENES_FILE,
            BUILD_FILE,
            SAFETY_FILE,
            PLAYTEST_FILE,
            OPS_FILE,
        ] {
            write_room_file(&room, file, "# Present\n").expect("write required file");
        }
        write_room_file(&room, BRIEF_FILE, "# Brief\n\n## Status\n").expect("write brief");
        write_room_file(&room, FLOORPLAN_FILE, "# Floorplan\n\n## Envelope\n")
            .expect("write floorplan");
        write_room_file(&room, PUZZLE_GRAPH_FILE, "# Graph\n\n## Nodes\n")
            .expect("write puzzle graph");

        let issues = room_contract_issues(&room).expect("room contract");

        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == BRIEF_FILE && heading == REQUIRED_BRIEF_HEADINGS[1]
        )));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == FLOORPLAN_FILE && heading == REQUIRED_FLOORPLAN_HEADINGS[1]
        )));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            RoomContractIssue::MissingHeading { file_name, heading }
                if file_name == PUZZLE_GRAPH_FILE && heading == REQUIRED_PUZZLE_GRAPH_HEADINGS[1]
        )));

        cleanup(room);
    }

    #[test]
    fn reads_typed_scene_and_build_docs() {
        let room = temp_room("typed-docs");
        fs::create_dir_all(&room).expect("create temp room");
        write_room_file(
            &room,
            SCENES_FILE,
            "\
# Scenes

## Beat cards

| Beat | Scene | Target min |
|---|---|---:|
| P1 latch | Dock | 3 |
",
        )
        .expect("write scenes");
        write_room_file(
            &room,
            BUILD_FILE,
            "\
# Build

## Bill of materials

| Component | Inventory ID | Beat IDs |
|---|---|---|
| hinge puzzle | MECH-001 | P1 |
",
        )
        .expect("write build");

        let scenes = read_scenes_doc(&room).expect("scenes doc");
        let build = read_build_doc(&room).expect("build doc");

        assert_eq!(scenes.require_beat_cards().unwrap()[0].name, "P1 latch");
        assert_eq!(build.require_components().unwrap()[0].name, "hinge puzzle");

        cleanup(room);
    }

    #[test]
    fn optional_docs_default_to_empty_when_missing() {
        let room = temp_room("optional-docs");
        fs::create_dir_all(&room).expect("create temp room");

        let build = read_optional_build_doc(&room);
        let playtest = read_optional_playtest_doc(&room);

        assert!(build.components.is_none());
        assert!(playtest.chaos_probes.is_none());
        assert!(playtest.into_evidence_probes().is_empty());

        cleanup(room);
    }

    #[test]
    fn active_room_paths_require_build_and_skip_template() {
        let root = temp_room("active-rooms");
        let active = root.join("0001-active");
        let no_build = root.join("0002-no-build");
        let template = root.join("TEMPLATE");
        fs::create_dir_all(&active).expect("create active");
        fs::create_dir_all(&no_build).expect("create no build");
        fs::create_dir_all(&template).expect("create template");
        write_room_file(&active, BUILD_FILE, "# Build\n").expect("write active build");
        write_room_file(&template, BUILD_FILE, "# Template\n").expect("write template");

        let rooms = active_room_paths(&root).expect("active rooms");

        assert_eq!(rooms, vec![active]);

        cleanup(root);
    }

    fn temp_room(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "amaze-harness-{label}-{}-{unique}",
            std::process::id()
        ))
    }

    fn cleanup(path: PathBuf) {
        let _ = fs::remove_dir_all(path);
    }
}
