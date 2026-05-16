use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cli::OpsArgs;
use crate::domain::{NpcCharacter, OperatorRotation, OpsDoc};
use crate::markdown::md_cell;
use crate::room::{active_room_paths, read_optional_ops_doc};

#[derive(Debug)]
struct RoomOpsReport {
    room: PathBuf,
    npc_characters: Vec<NpcCharacter>,
    operator_rotations: Vec<OperatorRotation>,
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
    }
}

fn print_room_summary(reports: &[RoomOpsReport]) {
    println!("## Room Summary");
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

fn print_rotation_capacity(reports: &[RoomOpsReport]) {
    let mut capacity: BTreeMap<u32, u32> = BTreeMap::new();
    for report in reports {
        for rotation in &report.operator_rotations {
            if let Ok(max_rooms) = rotation.max_rooms.parse::<u32>() {
                *capacity.entry(max_rooms).or_default() += 1;
            }
        }
    }

    println!("## Rotation Capacity");
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
    println!("## NPC and Operator Voices");
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
    println!("## Multi-room Operator Rotation");
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

    println!("## Dedicated Staff Trigger Themes");
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
