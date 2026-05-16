use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cli::ScoreArgs;
use crate::domain::{BuildDoc, PlaytestDoc, ScenesDoc};
use crate::evidence::{evidence_status_rank, normalize_evidence_status};
use crate::markdown::md_cell;
use crate::room::{
    active_room_paths, read_build_doc, read_optional_ops_doc, read_optional_playtest_doc,
    read_room_file, read_scenes_doc, room_contract_issues, BRIEF_FILE, FLOORPLAN_FILE, OPS_FILE,
    SAFETY_FILE,
};
use crate::rules::unlocks::{check_unlock_readiness, is_tbd};

#[derive(Debug)]
struct DimensionScore {
    dimension: &'static str,
    score: u32,
    max: u32,
    evidence: String,
}

#[derive(Debug)]
struct RoomScore {
    room: PathBuf,
    total: u32,
    max: u32,
    dimensions: Vec<DimensionScore>,
    revision_moves: Vec<String>,
}

pub(crate) fn score_room_pack(args: ScoreArgs) -> Result<(), String> {
    let rooms = score_targets(&args)?;
    let mut scores = Vec::new();

    for room in rooms {
        let target_minutes = args
            .target_minutes
            .unwrap_or_else(|| infer_target_minutes(&room));
        scores.push(score_room(&room, target_minutes)?);
    }

    println!("# AMAZE Draft Scorecard");
    println!();
    println!("| Room | Score | Top revision moves |");
    println!("|---|---:|---|");
    for score in &scores {
        println!(
            "| {} | {}/{} | {} |",
            md_cell(&score.room.display().to_string()),
            score.total,
            score.max,
            md_cell(&score.revision_moves.join("; "))
        );
    }

    if scores.len() > 1 {
        let average =
            scores.iter().map(|score| score.total).sum::<u32>() as f64 / scores.len() as f64;
        println!();
        println!("| Portfolio metric | Value |");
        println!("|---|---:|");
        println!("| Rooms scored | {} |", scores.len());
        println!("| Average score | {:.1} |", average);
        print_portfolio_breakdown(&scores);
    }

    if scores.len() == 1 {
        print_dimension_breakdown(&scores[0]);
    }

    Ok(())
}

fn score_targets(args: &ScoreArgs) -> Result<Vec<PathBuf>, String> {
    if let Some(room) = &args.room {
        return Ok(vec![room.clone()]);
    }

    let rooms_root = args
        .rooms
        .as_deref()
        .ok_or_else(|| "score requires --room <path> or --rooms <rooms-root>".to_string())?;
    active_room_paths(rooms_root)
}

fn score_room(room: &Path, target_minutes: u32) -> Result<RoomScore, String> {
    let scenes = read_scenes_doc(room)?;
    let build = read_build_doc(room)?;
    let playtest = read_optional_playtest_doc(room);
    let ops = read_optional_ops_doc(room);
    let beat_cards = scenes.require_beat_cards()?;
    let contract_failures = room_contract_issues(room)?.len();
    let unlock_failures = check_unlock_readiness(
        beat_cards,
        scenes.transformation_states.as_deref(),
        scenes.unlock_paths.as_deref(),
    );

    let dimensions = vec![
        score_theme(room),
        score_puzzle_graph(&scenes, contract_failures, unlock_failures.len()),
        score_physical(&scenes, &build),
        score_spatial(room),
        score_purpose_mapping(&scenes),
        score_safety(room),
        score_operator(room, &scenes, &playtest, &ops),
        score_throughput(&scenes, target_minutes),
        score_group_game(&scenes),
        score_delight(&scenes),
    ];
    let total = dimensions.iter().map(|dimension| dimension.score).sum();
    let max = dimensions.iter().map(|dimension| dimension.max).sum();
    let revision_moves = revision_moves(&dimensions, &unlock_failures);

    Ok(RoomScore {
        room: room.to_path_buf(),
        total,
        max,
        dimensions,
        revision_moves,
    })
}

fn infer_target_minutes(room: &Path) -> u32 {
    let brief = read_room_file(room, BRIEF_FILE).unwrap_or_default();
    for line in brief.lines() {
        let lower = line.to_lowercase();
        if lower.contains("target session") {
            if let Some(minutes) = first_number(line) {
                return minutes;
            }
        }
    }
    45
}

fn first_number(line: &str) -> Option<u32> {
    let mut digits = String::new();
    for ch in line.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if !digits.is_empty() {
            break;
        }
    }
    digits.parse().ok()
}

fn print_dimension_breakdown(score: &RoomScore) {
    println!();
    println!("## Dimension Breakdown");
    println!();
    println!("| Dimension | Score | Evidence |");
    println!("|---|---:|---|");
    for dimension in &score.dimensions {
        println!(
            "| {} | {}/{} | {} |",
            dimension.dimension,
            dimension.score,
            dimension.max,
            md_cell(&dimension.evidence)
        );
    }
}

fn print_portfolio_breakdown(scores: &[RoomScore]) {
    println!();
    println!("## Portfolio Dimension Averages");
    println!();
    println!("| Dimension | Average | Lowest room |");
    println!("|---|---:|---|");
    for dimension in dimension_names(scores) {
        let rows: Vec<(&RoomScore, &DimensionScore)> = scores
            .iter()
            .filter_map(|score| {
                score
                    .dimensions
                    .iter()
                    .find(|candidate| candidate.dimension == dimension)
                    .map(|dimension_score| (score, dimension_score))
            })
            .collect();
        if rows.is_empty() {
            continue;
        }
        let average_percent = rows
            .iter()
            .map(|(_, dimension_score)| {
                dimension_score.score as f64 * 100.0 / dimension_score.max.max(1) as f64
            })
            .sum::<f64>()
            / rows.len() as f64;
        let Some((lowest_room, lowest_dimension)) = rows
            .into_iter()
            .min_by_key(|(_, score)| score.score * 100 / score.max.max(1))
        else {
            continue;
        };
        println!(
            "| {} | {:.0}% | {} ({}/{}) |",
            dimension,
            average_percent,
            md_cell(&lowest_room.room.display().to_string()),
            lowest_dimension.score,
            lowest_dimension.max
        );
    }

    let pressure = revision_pressure(scores);
    if !pressure.is_empty() {
        println!();
        println!("## Repeated Revision Pressure");
        println!();
        println!("| Revision move | Rooms |");
        println!("|---|---:|");
        for (move_text, count) in pressure.into_iter().take(5) {
            println!("| {} | {} |", md_cell(&move_text), count);
        }
    }
}

fn dimension_names(scores: &[RoomScore]) -> Vec<&'static str> {
    let mut names = Vec::new();
    for score in scores {
        for dimension in &score.dimensions {
            if !names.contains(&dimension.dimension) {
                names.push(dimension.dimension);
            }
        }
    }
    names
}

fn revision_pressure(scores: &[RoomScore]) -> Vec<(String, u32)> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    for score in scores {
        for revision_move in &score.revision_moves {
            *counts.entry(revision_move.clone()).or_default() += 1;
        }
    }
    let mut pressure: Vec<(String, u32)> = counts.into_iter().collect();
    pressure.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    pressure
}

fn score_theme(room: &Path) -> DimensionScore {
    let brief = read_room_file(room, BRIEF_FILE).unwrap_or_default();
    let score = if brief.to_lowercase().contains("concept") {
        7
    } else {
        5
    };
    dimension(
        "Theme and story",
        score,
        8,
        "BRIEF concept and room premise present",
    )
}

fn score_puzzle_graph(
    scenes: &ScenesDoc,
    contract_failures: usize,
    unlock_failures: usize,
) -> DimensionScore {
    let beat_count = scenes.beat_cards.as_ref().map_or(0, Vec::len);
    let unlock_count = scenes.unlock_paths.as_ref().map_or(0, Vec::len);
    let mut score: u32 = if beat_count >= 5 { 10 } else { 7 };
    if unlock_count >= 2 {
        score += 2;
    }
    score = score.saturating_sub((contract_failures as u32).min(4));
    score = score.saturating_sub((unlock_failures as u32).min(6));
    dimension(
        "Puzzle graph",
        score.min(12),
        12,
        format!("{beat_count} beats, {unlock_count} unlock paths, {unlock_failures} unlock issues"),
    )
}

fn score_physical(scenes: &ScenesDoc, build: &BuildDoc) -> DimensionScore {
    let transformations = scenes.transformation_states.as_ref().map_or(0, Vec::len);
    let components = build.components.as_ref().map_or(0, Vec::len);
    let recoverable = build.components.as_ref().map_or(0, |components| {
        components
            .iter()
            .filter(|component| {
                !is_tbd(&component.bypass)
                    || !is_tbd(&component.admin_recovery)
                    || component.spare_qty.unwrap_or(0) > 0
            })
            .count()
    });
    let score = if components == 0 {
        4
    } else {
        8 + (transformations >= 4) as u32 * 2 + (recoverable == components) as u32 * 2
    };
    dimension(
        "Physical interaction and reliability",
        score.min(14),
        14,
        format!("{components} BOM rows, {transformations} transformations, {recoverable} recoverable components"),
    )
}

fn score_spatial(room: &Path) -> DimensionScore {
    let floorplan = read_room_file(room, FLOORPLAN_FILE).unwrap_or_default();
    let score = if floorplan.contains("## Egress and safety paths")
        && floorplan.contains("## Accessibility and reach")
    {
        8
    } else {
        5
    };
    dimension(
        "Spatial flow",
        score,
        10,
        "floorplan declares egress and accessibility",
    )
}

fn score_purpose_mapping(scenes: &ScenesDoc) -> DimensionScore {
    let transformations = scenes.transformation_states.as_ref().map_or(0, Vec::len);
    let unlocks = scenes.unlock_paths.as_ref().map_or(0, Vec::len);
    let score = if transformations >= 5 && unlocks >= 2 {
        7
    } else {
        5
    };
    dimension(
        "Purpose mapping and integration",
        score,
        8,
        format!("{transformations} visible state changes mapped to {unlocks} unlock paths"),
    )
}

fn score_safety(room: &Path) -> DimensionScore {
    let safety = read_room_file(room, SAFETY_FILE)
        .unwrap_or_default()
        .to_lowercase();
    let score = if safety.contains("egress")
        && safety.contains("accessibility")
        && safety.contains("operator")
    {
        15
    } else {
        10
    };
    dimension(
        "Safety and accessibility",
        score,
        18,
        "SAFETY covers egress, accessibility, and operator controls",
    )
}

fn score_operator(
    room: &Path,
    scenes: &ScenesDoc,
    playtest: &PlaytestDoc,
    ops_doc: &crate::domain::OpsDoc,
) -> DimensionScore {
    let ops = read_room_file(room, OPS_FILE).unwrap_or_default();
    let acceleration_count = scenes.unlock_paths.as_ref().map_or(0, |paths| {
        paths
            .iter()
            .filter(|path| !is_tbd(&path.operator_acceleration))
            .count()
    });
    let evidence_count = playtest.bench_evidence.len()
        + playtest.admin_evidence.len()
        + playtest.chaos_evidence.len();
    let npc_count = ops_doc.npc_characters.as_ref().map_or(0, Vec::len);
    let rotation_count = ops_doc.operator_rotations.as_ref().map_or(0, Vec::len);
    let passed_evidence = playtest
        .bench_evidence
        .iter()
        .chain(playtest.admin_evidence.iter())
        .chain(playtest.chaos_evidence.iter())
        .filter(|probe| evidence_status_rank(&normalize_evidence_status(&probe.raw_status)) >= 3)
        .count();
    let score = 6
        + (ops.contains("## Failure modes") as u32)
        + (acceleration_count > 0) as u32
        + (passed_evidence > 0) as u32
        + (npc_count > 0) as u32
        + (rotation_count > 0) as u32;
    dimension(
        "Operator support",
        score.min(10),
        10,
        format!("{acceleration_count} acceleration paths, {npc_count} NPC/operator voices, {rotation_count} multi-room rotation models, {passed_evidence}/{evidence_count} evidence probes passed"),
    )
}

fn score_throughput(scenes: &ScenesDoc, target_minutes: u32) -> DimensionScore {
    let beat_cards = scenes.beat_cards.as_deref().unwrap_or_default();
    let target_total: u32 = beat_cards.iter().map(|beat| beat.target_or_default()).sum();
    let slow_total: u32 = beat_cards.iter().map(|beat| beat.slow_or_default()).sum();
    let score = if slow_total <= target_minutes {
        8
    } else if target_total <= target_minutes {
        6
    } else {
        3
    };
    dimension(
        "Throughput and timing",
        score,
        8,
        format!("target total {target_total} min, slow total {slow_total} min, session target {target_minutes} min"),
    )
}

fn score_group_game(scenes: &ScenesDoc) -> DimensionScore {
    let teams = scenes.team_probes.as_ref().map_or(0, Vec::len);
    let behaviors = scenes.behavior_probes.as_ref().map_or(0, Vec::len);
    let score = if teams >= 6 && behaviors >= 5 { 7 } else { 5 };
    dimension(
        "Group-game quality",
        score,
        8,
        format!("{teams} team probes, {behaviors} behavior probes"),
    )
}

fn score_delight(scenes: &ScenesDoc) -> DimensionScore {
    let transformations = scenes.transformation_states.as_ref().map_or(0, Vec::len);
    let score = if transformations >= 5 { 3 } else { 2 };
    dimension(
        "Delight and memorability",
        score,
        4,
        format!("{transformations} visible reveals/state changes"),
    )
}

fn revision_moves(dimensions: &[DimensionScore], unlock_failures: &[String]) -> Vec<String> {
    let mut moves: Vec<String> = unlock_failures.iter().take(2).cloned().collect();
    let mut low_dimensions: Vec<&DimensionScore> = dimensions.iter().collect();
    low_dimensions.sort_by(|left, right| {
        let left_percent = left.score * 100 / left.max.max(1);
        let right_percent = right.score * 100 / right.max.max(1);
        left_percent.cmp(&right_percent)
    });
    for dimension in low_dimensions.into_iter().take(3) {
        moves.push(format!("improve {}", dimension.dimension));
    }
    moves.sort();
    moves.dedup();
    moves.truncate(3);
    moves
}

fn dimension(
    dimension: &'static str,
    score: u32,
    max: u32,
    evidence: impl Into<String>,
) -> DimensionScore {
    DimensionScore {
        dimension,
        score: score.min(max),
        max,
        evidence: evidence.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::revision_moves;
    use super::DimensionScore;

    #[test]
    fn revision_moves_include_lowest_dimensions() {
        let dimensions = vec![
            DimensionScore {
                dimension: "A",
                score: 9,
                max: 10,
                evidence: String::new(),
            },
            DimensionScore {
                dimension: "B",
                score: 3,
                max: 10,
                evidence: String::new(),
            },
        ];

        assert!(revision_moves(&dimensions, &[])
            .iter()
            .any(|move_text| move_text.contains("B")));
    }

    #[test]
    fn extracts_first_number_from_target_line() {
        assert_eq!(
            super::first_number("- Target session time: 90 minutes"),
            Some(90)
        );
    }

    #[test]
    fn counts_repeated_revision_pressure() {
        let scores = vec![
            room_score("room-a", vec!["improve Operator support"]),
            room_score(
                "room-b",
                vec!["improve Operator support", "improve Spatial flow"],
            ),
        ];

        assert_eq!(
            super::revision_pressure(&scores)[0],
            ("improve Operator support".to_string(), 2)
        );
    }

    fn room_score(room: &str, moves: Vec<&str>) -> super::RoomScore {
        super::RoomScore {
            room: std::path::PathBuf::from(room),
            total: 0,
            max: 100,
            dimensions: vec![DimensionScore {
                dimension: "Operator support",
                score: 5,
                max: 10,
                evidence: String::new(),
            }],
            revision_moves: moves.into_iter().map(str::to_string).collect(),
        }
    }
}
