use std::path::{Path, PathBuf};

use crate::domain::{BuildDoc, Component, PartDiagram};
use crate::room::read_build_doc;

#[derive(Debug)]
pub(crate) struct VisualReportRow {
    pub(crate) room: String,
    pub(crate) component: String,
    pub(crate) criticality: String,
    pub(crate) diagram: String,
    pub(crate) status: String,
    pub(crate) gap: String,
}

#[derive(Debug)]
pub(crate) struct VisualFilter<'a> {
    pub(crate) open_c5: bool,
    pub(crate) status: Option<&'a str>,
    pub(crate) gap: Option<&'a str>,
}

pub(crate) fn filter_visual_rows(
    rows: Vec<VisualReportRow>,
    filter: &VisualFilter<'_>,
) -> Vec<VisualReportRow> {
    rows.into_iter()
        .filter(|row| !filter.open_c5 || (row.status != "build-ready" && is_c5(&row.criticality)))
        .filter(|row| filter.status.is_none_or(|status| row.status == status))
        .filter(|row| {
            filter
                .gap
                .is_none_or(|gap| row.gap.to_lowercase().contains(&gap.to_lowercase()))
        })
        .collect()
}

pub(crate) fn visual_report_rows(room: &Path) -> Result<Vec<VisualReportRow>, String> {
    let build_doc = read_build_doc(room)?;
    let components = build_doc.require_components()?;

    let mut rows = Vec::new();

    for component in components {
        if !requires_visual_reference(component) {
            continue;
        }

        let direct_reference = is_usable_visual_reference(room, &component.visual_reference);
        let visual_row = matching_visual_row(room, component, build_doc.part_diagrams.as_deref());
        let diagram = visual_row
            .map(|visual| visual.diagram.clone())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| {
                if direct_reference {
                    component.visual_reference.clone()
                } else {
                    "missing".to_string()
                }
            });

        let (status, gap) = if let Some(visual) = visual_row {
            match visual_readiness(visual) {
                VisualReadiness::BuildReady => ("build-ready", "none".to_string()),
                VisualReadiness::Draft(gap) => ("draft", gap),
                VisualReadiness::Blocked(reason) => ("blocked", reason),
            }
        } else if direct_reference {
            (
                "draft",
                "direct diagram reference has no room-specific Part diagrams row".to_string(),
            )
        } else {
            ("blocked", "missing visual reference".to_string())
        };

        rows.push(VisualReportRow {
            room: room.display().to_string(),
            component: component.name.clone(),
            criticality: component.criticality.clone(),
            diagram,
            status: status.to_string(),
            gap,
        });
    }

    Ok(rows)
}

pub(crate) fn check_visual_references(
    room: &Path,
    build: &str,
    failures: &mut Vec<String>,
    notes: &mut Vec<String>,
) {
    let build_doc = BuildDoc::from_markdown(build);
    let Some(components) = build_doc.components.as_ref() else {
        return;
    };

    for component in components {
        if !requires_visual_reference(component) {
            continue;
        }
        let direct_reference = is_usable_visual_reference(room, &component.visual_reference);
        let visual_row = matching_visual_row(room, component, build_doc.part_diagrams.as_deref());

        if direct_reference || visual_row.is_some() {
            if let Some(visual) = visual_row {
                match visual_readiness(visual) {
                    VisualReadiness::BuildReady => {}
                    VisualReadiness::Draft(gap) => notes.push(format!(
                        "visual readiness draft for {}: {gap}",
                        component.name
                    )),
                    VisualReadiness::Blocked(reason) => failures.push(format!(
                        "BUILD.md Part diagrams row not reviewable for {} ({}): {reason}",
                        component.name, component.inventory_id
                    )),
                }
            } else {
                notes.push(format!(
                    "visual readiness draft for {}: direct diagram reference has no room-specific Part diagrams row",
                    component.name
                ));
            }
            continue;
        }

        failures.push(format!(
            "BUILD.md C4-C5 BOM row missing visual reference: {} ({})",
            component.name, component.inventory_id
        ));
    }
}

pub(crate) enum VisualReadiness {
    BuildReady,
    Draft(String),
    Blocked(String),
}

pub(crate) fn requires_visual_reference(component: &Component) -> bool {
    let criticality = component.criticality.to_lowercase();
    criticality.contains("c4")
        || criticality.contains("c5")
        || criticality.contains("required")
        || criticality.contains("showstopper")
}

pub(crate) fn matching_visual_row<'a>(
    room: &Path,
    component: &Component,
    part_diagrams: Option<&'a [PartDiagram]>,
) -> Option<&'a PartDiagram> {
    part_diagrams?
        .iter()
        .filter(|visual| is_usable_visual_reference(room, &visual.diagram))
        .filter_map(|visual| {
            let score = visual_match_score(visual, component);
            (score > 0).then_some((score, visual))
        })
        .max_by_key(|(score, _)| *score)
        .map(|(_, visual)| visual)
}

pub(crate) fn visual_readiness(visual: &PartDiagram) -> VisualReadiness {
    if is_blank_or_tbd(&visual.proof) {
        return VisualReadiness::Blocked("missing `What it proves` evidence".to_string());
    }

    if is_missing_visual_gap(&visual.build_readiness_gap) {
        return VisualReadiness::Blocked(
            "missing explicit `Missing before build readiness` gap".to_string(),
        );
    }

    let normalized = visual.build_readiness_gap.to_lowercase();
    if normalized.contains("build-ready")
        || normalized.contains("build ready")
        || normalized.contains("none")
        || normalized.contains("complete")
    {
        VisualReadiness::BuildReady
    } else {
        VisualReadiness::Draft(visual.build_readiness_gap.clone())
    }
}

pub(crate) fn is_c5(criticality: &str) -> bool {
    let lower = criticality.to_lowercase();
    lower.contains("c5") || lower.contains("showstopper")
}

pub(crate) fn criticality_rank(criticality: &str) -> u8 {
    if is_c5(criticality) {
        0
    } else if criticality.to_lowercase().contains("c4") {
        1
    } else {
        2
    }
}

pub(crate) fn visual_status_rank(status: &str) -> u8 {
    match status {
        "blocked" => 0,
        "draft" => 1,
        "build-ready" => 2,
        _ => 3,
    }
}

pub(crate) fn sprint_gap_filter(gap: &str) -> String {
    [
        "pouch",
        "cabinet",
        "enclosure",
        "griddle",
        "table",
        "token",
        "sightline",
        "balance",
        "switch",
        "tile",
    ]
    .iter()
    .find(|keyword| gap.to_lowercase().contains(**keyword))
    .unwrap_or(&gap)
    .to_string()
}

fn is_blank_or_tbd(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.is_empty()
        || matches!(
            trimmed.to_lowercase().as_str(),
            "tbd" | "none" | "n/a" | "na" | "missing"
        )
}

fn is_missing_visual_gap(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.is_empty()
        || matches!(
            trimmed.to_lowercase().as_str(),
            "tbd" | "n/a" | "na" | "missing"
        )
}

pub(crate) fn visual_match_score(visual: &PartDiagram, component: &Component) -> u32 {
    let visual_scope = visual.scope.to_lowercase();
    if visual_scope == "tbd" {
        return 0;
    }

    let component_text = format!(
        "{} {} {}",
        component.inventory_id, component.name, component.beat_ids
    )
    .to_lowercase();

    let keyword_score = visual_scope
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
        .filter(|token| is_visual_keyword(token))
        .filter(|token| visual_token_matches_component(token, &component_text))
        .count() as u32
        * 10;

    let component_beats = beat_ids(&component.beat_ids);
    let visual_beats = beat_ids(&visual_scope);
    let beat_score = component_beats
        .iter()
        .filter(|beat| visual_beats.iter().any(|visual_beat| visual_beat == *beat))
        .count() as u32
        * 20;

    keyword_score + beat_score
}

fn visual_token_matches_component(token: &str, component_text: &str) -> bool {
    component_text.contains(token)
        || token
            .strip_suffix('s')
            .is_some_and(|singular| singular.len() >= 4 && component_text.contains(singular))
}

fn is_visual_keyword(token: &str) -> bool {
    token.len() >= 4
        && !matches!(
            token,
            "beat"
                | "device"
                | "room"
                | "rooms"
                | "final"
                | "card"
                | "cards"
                | "board"
                | "public"
                | "simple"
                | "full"
                | "lite"
        )
}

pub(crate) fn beat_ids(text: &str) -> Vec<String> {
    let mut beats = Vec::new();
    for token in text
        .to_uppercase()
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
    {
        if let Some((left, right)) = token.split_once('-') {
            if let (Some(start), Some(end)) = (beat_number(left), beat_number(right)) {
                for number in start.min(end)..=start.max(end) {
                    beats.push(format!("P{number}"));
                }
            }
        }
    }

    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index].eq_ignore_ascii_case(&'p') {
            let start = index;
            index += 1;
            while index < chars.len() && chars[index].is_ascii_digit() {
                index += 1;
            }
            if index > start + 1 {
                beats.push(
                    chars[start..index]
                        .iter()
                        .collect::<String>()
                        .to_uppercase(),
                );
                continue;
            }
        }
        index += 1;
    }

    beats.sort();
    beats.dedup();
    beats
}

fn beat_number(token: &str) -> Option<u32> {
    token.strip_prefix('P')?.parse::<u32>().ok()
}

pub(crate) fn is_usable_visual_reference(room: &Path, text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty()
        || matches!(
            trimmed.to_lowercase().as_str(),
            "tbd" | "none" | "n/a" | "na" | "missing"
        )
    {
        return false;
    }

    diagram_paths(trimmed)
        .iter()
        .any(|path| visual_path_exists(room, path))
}

fn visual_path_exists(room: &Path, path: &str) -> bool {
    let normalized = normalize_visual_path(path);
    room.join(path).exists()
        || Path::new(path).exists()
        || room.join(&normalized).exists()
        || normalized.exists()
}

fn normalize_visual_path(path: &str) -> PathBuf {
    path.split(['\\', '/']).collect()
}

fn diagram_paths(text: &str) -> Vec<String> {
    text.split(|ch: char| ch.is_whitespace() || ch == ',' || ch == ';')
        .map(|token| {
            token.trim_matches(|ch: char| ch == '`' || ch == '"' || ch == '\'' || ch == '.')
        })
        .filter(|token| token.to_lowercase().ends_with(".excalidraw"))
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::domain::{Component, PartDiagram};

    use super::{
        beat_ids, normalize_visual_path, requires_visual_reference, visual_match_score,
        visual_readiness, VisualReadiness,
    };

    #[test]
    fn extracts_beat_ids_from_ranges_and_lists() {
        assert_eq!(
            beat_ids("P2/P3 and P4, P2-P5"),
            vec![
                "P2".to_string(),
                "P3".to_string(),
                "P4".to_string(),
                "P5".to_string()
            ]
        );
    }

    #[test]
    fn normalizes_visual_paths_across_platforms() {
        assert_eq!(
            normalize_visual_path("components\\diagrams\\part.excalidraw"),
            PathBuf::from_iter(["components", "diagrams", "part.excalidraw"])
        );
        assert_eq!(
            normalize_visual_path("components/diagrams/part.excalidraw"),
            PathBuf::from_iter(["components", "diagrams", "part.excalidraw"])
        );
    }

    #[test]
    fn visual_row_matches_by_beat_overlap() {
        let component = component(&[("Beat IDs", "P2-P5"), ("Component", "proof tokens")]);

        let visual = visual("P4 overlays", "diagram.excalidraw", "proof", "none");

        assert!(visual_match_score(&visual, &component) > 0);
    }

    #[test]
    fn visual_match_prefers_keywords_over_broad_beat_overlap() {
        let component = component(&[("Beat IDs", "P3"), ("Component", "balance plinth")]);

        let broad = visual("P2/P3/P4 proof tokens", "broad.excalidraw", "proof", "none");

        let precise = visual("P3 balance plinth", "precise.excalidraw", "proof", "none");

        assert!(visual_match_score(&precise, &component) > visual_match_score(&broad, &component));
    }

    #[test]
    fn visual_match_ignores_generic_board_keyword() {
        let component = component(&[
            ("Beat IDs", "P2"),
            ("Component", "portrait truth-window board"),
        ]);

        let generic = visual("P5 verdict board", "generic.excalidraw", "proof", "none");

        let specific = visual("P4 room windows", "specific.excalidraw", "proof", "none");

        assert!(
            visual_match_score(&specific, &component) > visual_match_score(&generic, &component)
        );
    }

    #[test]
    fn visual_readiness_tracks_draft_gaps() {
        let visual = visual(
            "P1 hinge",
            "hinge.excalidraw",
            "reset and failure path",
            "final hinge dimensions",
        );

        assert!(matches!(
            visual_readiness(&visual),
            VisualReadiness::Draft(_)
        ));
    }

    #[test]
    fn visual_readiness_blocks_empty_gap() {
        let visual = visual(
            "P1 hinge",
            "hinge.excalidraw",
            "reset and failure path",
            "TBD",
        );

        assert!(matches!(
            visual_readiness(&visual),
            VisualReadiness::Blocked(_)
        ));
    }

    #[test]
    fn visual_readiness_accepts_build_ready_gap() {
        let visual = visual("P1 hinge", "hinge.excalidraw", "reset path", "build-ready");

        assert!(matches!(
            visual_readiness(&visual),
            VisualReadiness::BuildReady
        ));
    }

    #[test]
    fn c4_c5_and_required_components_need_visuals() {
        assert!(requires_visual_reference(&component(&[(
            "Criticality",
            "C4 required",
        )])));
        assert!(requires_visual_reference(&component(&[(
            "Criticality",
            "C5 showstopper",
        )])));
        assert!(!requires_visual_reference(&component(&[(
            "Criticality",
            "C2 cosmetic",
        )])));
    }

    fn component(values: &[(&str, &str)]) -> Component {
        let row = values
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        Component::from_row(&row)
    }

    fn visual(scope: &str, diagram: &str, proof: &str, gap: &str) -> PartDiagram {
        PartDiagram {
            scope: scope.to_string(),
            diagram: diagram.to_string(),
            proof: proof.to_string(),
            build_readiness_gap: gap.to_string(),
        }
    }
}
