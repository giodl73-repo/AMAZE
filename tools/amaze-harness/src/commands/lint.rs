use std::collections::HashMap;
use std::path::{Path, PathBuf};

use mdpath::document::{ParsedDocument, ParsedElement};
use mdpath::heading::normalize_heading;
use mdpath::parser::parse_document;

use crate::cli::LintArgs;
use crate::markdown::md_cell;
use crate::room::{
    active_room_paths, read_room_file, room_contract_issues, room_file_path, RoomContractIssue,
    BUILD_FILE, OPS_FILE, PLAYTEST_FILE, REQUIRED_ROOM_FILES, SCENES_FILE,
};

struct SectionTableRule {
    file_name: &'static str,
    heading: &'static str,
    required_headers: &'static [&'static str],
}

const SECTION_TABLE_RULES: &[SectionTableRule] = &[
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Scene list",
        required_headers: &["Scene", "Purpose"],
    },
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Beat cards",
        required_headers: &["Beat", "Scene", "Target min"],
    },
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Team archetype probes",
        required_headers: &["Team archetype", "Scene/beat probe"],
    },
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Behavior probes",
        required_headers: &["Behavior", "Team/persona"],
    },
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Transformation states",
        required_headers: &[
            "ID",
            "Beat",
            "Object/space",
            "From state",
            "Trigger",
            "To state",
        ],
    },
    SectionTableRule {
        file_name: SCENES_FILE,
        heading: "## Unlock paths",
        required_headers: &[
            "Path",
            "Beats",
            "Unlocks",
            "Fast coherence",
            "Slow coherence",
        ],
    },
    SectionTableRule {
        file_name: BUILD_FILE,
        heading: "## Bill of materials",
        required_headers: &["Inventory ID", "Component", "Beat IDs", "Criticality"],
    },
    SectionTableRule {
        file_name: BUILD_FILE,
        heading: "## Part diagrams",
        required_headers: &[
            "Diagram",
            "What it proves",
            "Missing before build readiness",
        ],
    },
    SectionTableRule {
        file_name: PLAYTEST_FILE,
        heading: "## Simulation and playtest runs",
        required_headers: &["Run ID", "Team archetype"],
    },
    SectionTableRule {
        file_name: PLAYTEST_FILE,
        heading: "## Variability matrix",
        required_headers: &["Team/behavior", "Reliability issue IDs"],
    },
    SectionTableRule {
        file_name: PLAYTEST_FILE,
        heading: "## Chaos protocol",
        required_headers: &["Probe ID", "Beat/device", "Status"],
    },
    SectionTableRule {
        file_name: PLAYTEST_FILE,
        heading: "## Device bench tests",
        required_headers: &["Test ID", "Device", "Status"],
    },
    SectionTableRule {
        file_name: PLAYTEST_FILE,
        heading: "## Admin replacement drills",
        required_headers: &["Test ID", "Component", "Result"],
    },
    SectionTableRule {
        file_name: OPS_FILE,
        heading: "## NPC and operator characters",
        required_headers: &[
            "NPC/voice",
            "Function",
            "Trigger",
            "Delivery mode",
            "Limits",
        ],
    },
    SectionTableRule {
        file_name: OPS_FILE,
        heading: "## Multi-room operator rotation",
        required_headers: &[
            "Scope",
            "Coverage model",
            "Max rooms",
            "Safe when",
            "Unsafe when",
            "Dedicated staff trigger",
        ],
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum LintIssue {
    RoomContract(RoomContractIssue),
    EmptyHeading {
        file_name: String,
        line: usize,
    },
    DuplicateHeadingAnchor {
        file_name: String,
        anchor: String,
        first_line: usize,
        line: usize,
    },
    HeadingLevelJump {
        file_name: String,
        line: usize,
        previous_level: usize,
        level: usize,
    },
    TableWidthMismatch {
        file_name: String,
        uri: String,
        line: usize,
        expected: usize,
        actual: usize,
    },
    MissingSectionTable {
        file_name: String,
        uri: String,
        heading: String,
    },
    MissingTableHeaders {
        file_name: String,
        uri: String,
        line: usize,
        heading: String,
        missing_headers: Vec<String>,
    },
}

impl LintIssue {
    fn message(&self) -> String {
        match self {
            Self::RoomContract(issue) => issue.message(),
            Self::EmptyHeading { file_name, line } => {
                format!("{file_name}:{line} empty Markdown heading")
            }
            Self::DuplicateHeadingAnchor {
                file_name,
                anchor,
                first_line,
                line,
            } => format!(
                "{file_name}:{line} duplicate heading anchor '{anchor}' first seen on line {first_line}"
            ),
            Self::HeadingLevelJump {
                file_name,
                line,
                previous_level,
                level,
            } => format!("{file_name}:{line} heading jumps from H{previous_level} to H{level}"),
            Self::TableWidthMismatch {
                file_name,
                uri,
                line,
                expected,
                actual,
            } => format!(
                "{file_name}:{line} {uri} table row has {actual} cells, expected {expected}"
            ),
            Self::MissingSectionTable {
                file_name,
                uri,
                heading,
            } => format!("{file_name} {uri} missing table in section {heading}"),
            Self::MissingTableHeaders {
                file_name,
                uri,
                line,
                heading,
                missing_headers,
            } => format!(
                "{}:{line} {uri} table in section {heading} missing headers: {}",
                file_name,
                missing_headers.join(", ")
            ),
        }
    }
}

pub(crate) fn lint_room_pack(args: LintArgs) -> Result<(), String> {
    let rooms = lint_targets(args)?;
    let mut failed = false;

    println!("# AMAZE Room Pack Lint");
    println!();
    println!("| Room | Status | Issues |");
    println!("|---|---|---:|");

    for room in rooms {
        let issues = lint_room(&room)?;
        if issues.is_empty() {
            println!("| {} | pass | 0 |", md_cell(&room.display().to_string()));
            continue;
        }

        failed = true;
        println!(
            "| {} | fail | {} |",
            md_cell(&room.display().to_string()),
            issues.len()
        );
        for issue in issues {
            eprintln!("- {}: {}", room.display(), issue.message());
        }
    }

    if failed {
        Err("room lint failed".to_string())
    } else {
        Ok(())
    }
}

fn lint_targets(args: LintArgs) -> Result<Vec<PathBuf>, String> {
    if let Some(room) = args.room {
        return Ok(vec![room]);
    }

    let rooms_root = args
        .rooms
        .ok_or_else(|| "lint requires --room <path> or --rooms <rooms-root>".to_string())?;
    active_room_paths(&rooms_root)
}

fn lint_room(room: &Path) -> Result<Vec<LintIssue>, String> {
    let mut issues: Vec<LintIssue> = room_contract_issues(room)?
        .into_iter()
        .map(LintIssue::RoomContract)
        .collect();

    for file_name in REQUIRED_ROOM_FILES {
        let file_path = room_file_path(room, file_name);
        if file_path.exists() {
            let markdown = read_room_file(room, file_name)?;
            issues.extend(lint_markdown_file(
                file_name,
                &md_uri_path(&file_path),
                &markdown,
            ));
        }
    }

    Ok(issues)
}

fn lint_markdown_file(file_name: &str, uri_path: &str, markdown: &str) -> Vec<LintIssue> {
    let doc = parse_document(markdown);
    let mut issues = Vec::new();
    let mut anchors: HashMap<&str, usize> = HashMap::new();
    let mut previous_level = 0;

    for heading in doc.headings.iter().filter(|heading| heading.level > 0) {
        if heading.text.trim().is_empty() {
            issues.push(LintIssue::EmptyHeading {
                file_name: file_name.to_string(),
                line: heading.line,
            });
        }

        if let Some(first_line) = anchors.insert(heading.anchor.as_str(), heading.line) {
            issues.push(LintIssue::DuplicateHeadingAnchor {
                file_name: file_name.to_string(),
                anchor: heading.anchor.clone(),
                first_line,
                line: heading.line,
            });
        }

        if previous_level > 0 && heading.level > previous_level + 1 {
            issues.push(LintIssue::HeadingLevelJump {
                file_name: file_name.to_string(),
                line: heading.line,
                previous_level,
                level: heading.level,
            });
        }
        previous_level = heading.level;
    }

    for element in &doc.elements {
        if let ParsedElement::Table(table) = element {
            let expected = table.headers.len();
            let uri = section_uri(
                uri_path,
                doc.headings
                    .get(table.heading_idx)
                    .map(|heading| heading.anchor.as_str())
                    .unwrap_or_default(),
            );
            if table.separator.len() != expected {
                issues.push(LintIssue::TableWidthMismatch {
                    file_name: file_name.to_string(),
                    uri: uri.clone(),
                    line: table.line_start + 1,
                    expected,
                    actual: table.separator.len(),
                });
            }
            for (offset, row) in table.rows.iter().enumerate() {
                if row.len() != expected {
                    issues.push(LintIssue::TableWidthMismatch {
                        file_name: file_name.to_string(),
                        uri: uri.clone(),
                        line: table.line_start + 2 + offset,
                        expected,
                        actual: row.len(),
                    });
                }
            }
        }
    }

    for rule in SECTION_TABLE_RULES
        .iter()
        .filter(|rule| rule.file_name == file_name)
    {
        issues.extend(lint_section_table_rule(file_name, uri_path, &doc, rule));
    }

    issues
}

fn lint_section_table_rule(
    file_name: &str,
    uri_path: &str,
    doc: &ParsedDocument,
    rule: &SectionTableRule,
) -> Vec<LintIssue> {
    let anchor = normalize_heading(rule.heading);
    let uri = section_uri(uri_path, &anchor);
    let Some((heading_idx, _)) = doc
        .headings
        .iter()
        .enumerate()
        .find(|(_, heading)| heading.level > 0 && heading.anchor == anchor)
    else {
        return Vec::new();
    };

    let Some(table) = doc
        .elements_in_section(heading_idx)
        .into_iter()
        .find_map(|element| {
            if let ParsedElement::Table(table) = element {
                Some(table)
            } else {
                None
            }
        })
    else {
        return vec![LintIssue::MissingSectionTable {
            file_name: file_name.to_string(),
            uri,
            heading: rule.heading.to_string(),
        }];
    };

    let missing_headers: Vec<String> = rule
        .required_headers
        .iter()
        .filter(|header| {
            !table
                .headers
                .iter()
                .any(|candidate| candidate.trim() == **header)
        })
        .map(|header| (*header).to_string())
        .collect();

    if missing_headers.is_empty() {
        Vec::new()
    } else {
        vec![LintIssue::MissingTableHeaders {
            file_name: file_name.to_string(),
            uri,
            line: table.line_start,
            heading: rule.heading.to_string(),
            missing_headers,
        }]
    }
}

fn section_uri(uri_path: &str, anchor: &str) -> String {
    if anchor.is_empty() {
        format!("md://{uri_path}")
    } else {
        format!("md://{uri_path}#{anchor}")
    }
}

fn md_uri_path(path: &Path) -> String {
    let relative = std::env::current_dir()
        .ok()
        .and_then(|cwd| path.strip_prefix(cwd).ok().map(Path::to_path_buf))
        .unwrap_or_else(|| path.to_path_buf());

    relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::{lint_markdown_file, LintIssue};

    #[test]
    fn lint_catches_mdpath_heading_and_table_issues() {
        let issues = lint_markdown_file(
            "SCENES.md",
            "rooms/example/SCENES.md",
            "\
# Room

### Jumped

## Repeated

## Repeated

| A | B |
|---|
| one | two | three |
",
        );

        assert!(issues
            .iter()
            .any(|issue| matches!(issue, LintIssue::HeadingLevelJump { line: 3, .. })));
        assert!(issues
            .iter()
            .any(|issue| matches!(issue, LintIssue::DuplicateHeadingAnchor { line: 7, .. })));
        assert!(issues
            .iter()
            .any(|issue| matches!(issue, LintIssue::TableWidthMismatch { line: 10, .. })));
        assert!(issues
            .iter()
            .any(|issue| matches!(issue, LintIssue::TableWidthMismatch { line: 11, .. })));
    }

    #[test]
    fn lint_applies_specific_section_table_rules_with_mdpath_locations() {
        let issues = lint_markdown_file(
            "SCENES.md",
            "rooms/example/SCENES.md",
            "\
# Room

## Beat cards

| Beat | Scene |
|---|---|
| P1 | Intro |
",
        );

        assert!(issues.iter().any(|issue| matches!(
            issue,
            LintIssue::MissingTableHeaders {
                uri,
                missing_headers,
                ..
            } if uri == "md://rooms/example/SCENES.md#beat-cards"
                && missing_headers == &vec!["Target min".to_string()]
        )));
    }
}
