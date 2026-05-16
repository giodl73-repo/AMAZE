use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cli::OpsArgs;
use crate::domain::{NpcCharacter, OperatorRotation, OpsDoc};
use crate::markdown::md_cell;
use crate::room::{active_room_paths, read_optional_ops_doc, read_room_file, BRIEF_FILE};

#[derive(Debug)]
struct RoomOpsReport {
    room: PathBuf,
    npc_characters: Vec<NpcCharacter>,
    operator_rotations: Vec<OperatorRotation>,
    target_minutes: u32,
}

pub(crate) fn ops_readiness_report(args: OpsArgs) -> Result<(), String> {
    let reports = ops_targets(&args)?
        .iter()
        .map(|room| room_ops_report(room))
        .collect::<Vec<_>>();

    if reports.len() == 1 {
        println!("# AMAZE Ops Readiness");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        println!("| Room | {} |", reports[0].room.display());
        println!();
        print_npc_rows(&reports, false);
        print_rotation_rows(&reports, false);
        print_room_summary(&reports);
    } else {
        println!("# AMAZE Ops Readiness Portfolio");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        let rooms_root = args
            .rooms
            .as_ref()
            .ok_or_else(|| "ops portfolio requires --rooms <rooms-root>".to_string())?;
        println!("| Rooms root | {} |", rooms_root.display());
        println!("| Rooms reported | {} |", reports.len());
        println!();
        print_room_summary(&reports);
        print_rotation_capacity(&reports);
        print_stagger_model(&reports, args.stagger_minutes, args.finale_minutes);
        print_npc_rows(&reports, true);
        print_rotation_rows(&reports, true);
        print_staffing_triggers(&reports);
    }

    Ok(())
}

fn ops_targets(args: &OpsArgs) -> Result<Vec<PathBuf>, String> {
    if let Some(room) = &args.room {
        return Ok(vec![room.clone()]);
    }

    let rooms_root = args
        .rooms
        .as_deref()
        .ok_or_else(|| "ops requires --room <path> or --rooms <rooms-root>".to_string())?;
    active_room_paths(rooms_root)
}

fn room_ops_report(room: &Path) -> RoomOpsReport {
    let OpsDoc {
        npc_characters,
        operator_rotations,
    } = read_optional_ops_doc(room);

    RoomOpsReport {
        room: room.to_path_buf(),
        npc_characters: npc_characters.unwrap_or_default(),
        operator_rotations: operator_rotations.unwrap_or_default(),
        target_minutes: infer_target_minutes(room),
    }
}

fn print_room_summary(reports: &[RoomOpsReport]) {
    println!("\n## Room Summary");
    println!();
    println!("| Room | NPC/voices | Rotation models | Max shared rooms | Readiness note |");
    println!("|---|---:|---:|---:|---|");
    if reports.is_empty() {
        println!("| none | 0 | 0 | 0 | no rooms reported |");
    }
    for report in reports {
        let max_rooms = report
            .operator_rotations
            .iter()
            .filter_map(|rotation| rotation.max_rooms.parse::<u32>().ok())
            .max()
            .unwrap_or(0);
        let note = if report.npc_characters.is_empty() || report.operator_rotations.is_empty() {
            "missing declared NPC/rotation layer"
        } else if max_rooms >= 3 {
            "supports 3-room portfolio model with constraints"
        } else {
            "supports shared-operator model with constraints"
        };
        println!(
            "| {} | {} | {} | {} | {} |",
            md_cell(&report.room.display().to_string()),
            report.npc_characters.len(),
            report.operator_rotations.len(),
            max_rooms,
            note
        );
    }
    println!();
}

#[derive(Debug)]
struct ScheduledRoom<'a> {
    report: &'a RoomOpsReport,
    start: u32,
    end: u32,
    finale_start: u32,
}

fn print_stagger_model(reports: &[RoomOpsReport], stagger_minutes: u32, finale_minutes: u32) {
    let schedule = schedule_rooms(reports, stagger_minutes, finale_minutes);
    let portfolio_capacity = shared_operator_capacity(reports);
    let peak_active = peak_overlap(
        &schedule
            .iter()
            .map(|room| (room.start, room.end))
            .collect::<Vec<_>>(),
    );
    let peak_finals = peak_overlap(
        &schedule
            .iter()
            .map(|room| (room.finale_start, room.end))
            .collect::<Vec<_>>(),
    );
    let status = if portfolio_capacity == 0 {
        "blocked: no rotation capacity declared"
    } else if peak_active > portfolio_capacity {
        "operator overload: increase stagger or add staff"
    } else if peak_finals > 1 {
        "finale pressure: avoid solo coverage during overlap"
    } else {
        "coherent for one shared operator under declared constraints"
    };

    println!("\n## Shared-operator Stagger Model");
    println!();
    println!("| Metric | Value |");
    println!("|---|---:|");
    println!("| Stagger minutes | {stagger_minutes} |");
    println!("| Finale focus window | {finale_minutes} |");
    println!("| Declared shared-room capacity | {portfolio_capacity} |");
    println!("| Peak active rooms | {peak_active} |");
    println!("| Peak overlapping finales | {peak_finals} |");
    println!("| Status | {} |", md_cell(status));
    println!();

    println!("| Start min | Room | Target min | Finale focus | Operator note |");
    println!("|---:|---|---:|---|---|");
    if schedule.is_empty() {
        println!("| 0 | none | 0 | none | no rooms scheduled |");
    }
    for room in schedule {
        let active_at_start = active_rooms_at(&room, reports, stagger_minutes);
        let note = if portfolio_capacity > 0 && active_at_start > portfolio_capacity {
            "starts above shared-operator capacity"
        } else if room.end.saturating_sub(room.start) > 60 {
            "long room: verify break/handoff coverage"
        } else {
            "standard stagger"
        };
        println!(
            "| {} | {} | {} | {}-{} | {} |",
            room.start,
            md_cell(&room.report.room.display().to_string()),
            room.report.target_minutes,
            room.finale_start,
            room.end,
            note
        );
    }
    println!();
}

fn schedule_rooms(
    reports: &[RoomOpsReport],
    stagger_minutes: u32,
    finale_minutes: u32,
) -> Vec<ScheduledRoom<'_>> {
    reports
        .iter()
        .enumerate()
        .map(|(index, report)| {
            let start = index as u32 * stagger_minutes;
            let end = start + report.target_minutes;
            let finale_start = end.saturating_sub(finale_minutes);
            ScheduledRoom {
                report,
                start,
                end,
                finale_start,
            }
        })
        .collect()
}

fn shared_operator_capacity(reports: &[RoomOpsReport]) -> u32 {
    reports
        .iter()
        .filter_map(|report| {
            report
                .operator_rotations
                .iter()
                .filter_map(|rotation| rotation.max_rooms.parse::<u32>().ok())
                .max()
        })
        .min()
        .unwrap_or(0)
}

fn peak_overlap(windows: &[(u32, u32)]) -> u32 {
    windows
        .iter()
        .flat_map(|(start, end)| [*start, *end])
        .map(|minute| {
            windows
                .iter()
                .filter(|(start, end)| *start <= minute && minute < *end)
                .count() as u32
        })
        .max()
        .unwrap_or(0)
}

fn active_rooms_at(
    room: &ScheduledRoom<'_>,
    reports: &[RoomOpsReport],
    stagger_minutes: u32,
) -> u32 {
    reports
        .iter()
        .enumerate()
        .filter(|(index, report)| {
            let start = *index as u32 * stagger_minutes;
            let end = start + report.target_minutes;
            start <= room.start && room.start < end
        })
        .count() as u32
}

fn infer_target_minutes(room: &Path) -> u32 {
    let brief = read_room_file(room, BRIEF_FILE).unwrap_or_default();
    for line in brief.lines() {
        if line.to_lowercase().contains("target session") {
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

#[cfg(test)]
mod tests {
    use super::{first_number, peak_overlap};

    #[test]
    fn peak_overlap_counts_active_windows() {
        assert_eq!(peak_overlap(&[(0, 45), (10, 55), (20, 65), (70, 100)]), 3);
        assert_eq!(peak_overlap(&[]), 0);
    }

    #[test]
    fn first_number_reads_target_minutes() {
        assert_eq!(
            first_number("- Target session time: 45-minute staffed-hour profile."),
            Some(45)
        );
        assert_eq!(first_number("no target here"), None);
    }
}

fn print_rotation_capacity(reports: &[RoomOpsReport]) {
    let mut capacity: BTreeMap<u32, u32> = BTreeMap::new();
    for report in reports {
        for rotation in &report.operator_rotations {
            if let Ok(max_rooms) = rotation.max_rooms.parse::<u32>() {
                *capacity.entry(max_rooms).or_default() += 1;
            }
        }
    }

    println!("\n## Rotation Capacity");
    println!();
    println!("| Max rooms | Rotation models |");
    println!("|---:|---:|");
    if capacity.is_empty() {
        println!("| 0 | 0 |");
    }
    for (max_rooms, count) in capacity {
        println!("| {max_rooms} | {count} |");
    }
    println!();
}

fn print_npc_rows(reports: &[RoomOpsReport], include_room: bool) {
    println!("\n## NPC and Operator Voices");
    println!();
    if include_room {
        println!(
            "| Room | NPC/voice | Function | Trigger | Delivery mode | Limits | Reset/ops burden |"
        );
        println!("|---|---|---|---|---|---|---|");
    } else {
        println!("| NPC/voice | Function | Trigger | Delivery mode | Limits | Reset/ops burden |");
        println!("|---|---|---|---|---|---|");
    }

    let mut printed = false;
    for report in reports {
        for npc in &report.npc_characters {
            printed = true;
            if include_room {
                println!(
                    "| {} | {} | {} | {} | {} | {} | {} |",
                    md_cell(&report.room.display().to_string()),
                    md_cell(&npc.name),
                    md_cell(&npc.function),
                    md_cell(&npc.trigger),
                    md_cell(&npc.delivery_mode),
                    md_cell(&npc.limits),
                    md_cell(&npc.reset_ops_burden)
                );
            } else {
                println!(
                    "| {} | {} | {} | {} | {} | {} |",
                    md_cell(&npc.name),
                    md_cell(&npc.function),
                    md_cell(&npc.trigger),
                    md_cell(&npc.delivery_mode),
                    md_cell(&npc.limits),
                    md_cell(&npc.reset_ops_burden)
                );
            }
        }
    }
    if !printed {
        if include_room {
            println!("| none | none | none | none | none | none | none |");
        } else {
            println!("| none | none | none | none | none | none |");
        }
    }
    println!();
}

fn print_rotation_rows(reports: &[RoomOpsReport], include_room: bool) {
    println!("\n## Multi-room Operator Rotation");
    println!();
    if include_room {
        println!("| Room | Scope | Coverage model | Max rooms | Safe when | Unsafe when | Handoff signal | Dedicated staff trigger |");
        println!("|---|---|---|---:|---|---|---|---|");
    } else {
        println!("| Scope | Coverage model | Max rooms | Safe when | Unsafe when | Handoff signal | Dedicated staff trigger |");
        println!("|---|---|---:|---|---|---|---|");
    }

    let mut printed = false;
    for report in reports {
        for rotation in &report.operator_rotations {
            printed = true;
            if include_room {
                println!(
                    "| {} | {} | {} | {} | {} | {} | {} | {} |",
                    md_cell(&report.room.display().to_string()),
                    md_cell(&rotation.scope),
                    md_cell(&rotation.coverage_model),
                    md_cell(&rotation.max_rooms),
                    md_cell(&rotation.safe_when),
                    md_cell(&rotation.unsafe_when),
                    md_cell(&rotation.handoff_signal),
                    md_cell(&rotation.dedicated_staff_trigger)
                );
            } else {
                println!(
                    "| {} | {} | {} | {} | {} | {} | {} |",
                    md_cell(&rotation.scope),
                    md_cell(&rotation.coverage_model),
                    md_cell(&rotation.max_rooms),
                    md_cell(&rotation.safe_when),
                    md_cell(&rotation.unsafe_when),
                    md_cell(&rotation.handoff_signal),
                    md_cell(&rotation.dedicated_staff_trigger)
                );
            }
        }
    }
    if !printed {
        if include_room {
            println!("| none | none | none | 0 | none | none | none | none |");
        } else {
            println!("| none | none | 0 | none | none | none | none |");
        }
    }
    println!();
}

fn print_staffing_triggers(reports: &[RoomOpsReport]) {
    let mut triggers: BTreeMap<&str, u32> = BTreeMap::new();
    for report in reports {
        for rotation in &report.operator_rotations {
            *triggers
                .entry(rotation.dedicated_staff_trigger.as_str())
                .or_default() += 1;
        }
    }
    let mut ranked: Vec<_> = triggers.into_iter().collect();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));

    println!("\n## Dedicated Staff Trigger Themes");
    println!();
    println!("| Trigger | Rooms/models |");
    println!("|---|---:|");
    if ranked.is_empty() {
        println!("| none | 0 |");
    }
    for (trigger, count) in ranked.into_iter().take(10) {
        println!("| {} | {count} |", md_cell(trigger));
    }
}
