use std::collections::BTreeMap;

use crate::cli::VisualsArgs;
use crate::markdown::md_cell;
use crate::room::active_room_paths;
use crate::rules::visual::{
    criticality_rank, filter_visual_rows, is_c5, sprint_gap_filter, visual_report_rows,
    visual_status_rank, VisualFilter, VisualReportRow,
};

pub(crate) fn visual_readiness_report(args: VisualsArgs) -> Result<(), String> {
    let filter = visual_filter(&args);
    if let Some(room) = args.room.as_ref() {
        let rows = filter_visual_rows(visual_report_rows(room)?, &filter);
        println!("# AMAZE Visual Readiness");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        println!("| Room | {} |", room.display());
        if args.open_c5 {
            println!("| Filter | open C5 |");
        }
        if let Some(status) = &args.status {
            println!("| Status filter | {status} |");
        }
        if let Some(gap) = &args.gap {
            println!("| Gap filter | {gap} |");
        }
        println!();
        print_visual_rows(&rows, false);
        return Ok(());
    }

    let rooms_root = args
        .rooms
        .as_ref()
        .ok_or_else(|| "visuals requires --room <path> or --rooms <rooms-root>".to_string())?;
    let room_paths = active_room_paths(rooms_root)?;
    let mut rows = Vec::new();
    for room in room_paths {
        rows.extend(visual_report_rows(&room)?);
    }
    let rows = filter_visual_rows(rows, &filter);

    println!("# AMAZE Visual Readiness Portfolio");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Rooms root | {} |", rooms_root.display());
    if args.open_c5 {
        println!("| Filter | open C5 |");
    }
    if let Some(status) = &args.status {
        println!("| Status filter | {status} |");
    }
    if let Some(gap) = &args.gap {
        println!("| Gap filter | {gap} |");
    }
    println!();
    print_visual_rows(&rows, true);

    Ok(())
}

fn visual_filter(args: &VisualsArgs) -> VisualFilter<'_> {
    VisualFilter {
        open_c5: args.open_c5,
        status: args.status.as_deref(),
        gap: args.gap.as_deref(),
    }
}

fn print_visual_rows(rows: &[VisualReportRow], include_room: bool) {
    let mut build_ready = 0;
    let mut draft = 0;
    let mut blocked = 0;
    let mut sorted: Vec<&VisualReportRow> = rows.iter().collect();
    sorted.sort_by(|left, right| {
        visual_status_rank(&left.status)
            .cmp(&visual_status_rank(&right.status))
            .then_with(|| {
                criticality_rank(&left.criticality).cmp(&criticality_rank(&right.criticality))
            })
            .then_with(|| left.room.cmp(&right.room))
            .then_with(|| left.component.cmp(&right.component))
    });

    println!("## C4-C5 Component Visuals");
    println!();
    if include_room {
        println!("| Room | Component | Criticality | Diagram | Status | Gap / blocker |");
        println!("|---|---|---|---|---|---|");
    } else {
        println!("| Component | Criticality | Diagram | Status | Gap / blocker |");
        println!("|---|---|---|---|---|");
    }

    for row in &sorted {
        match row.status.as_str() {
            "build-ready" => build_ready += 1,
            "draft" => draft += 1,
            "blocked" => blocked += 1,
            _ => {}
        }

        if include_room {
            println!(
                "| {} | {} | {} | {} | {} | {} |",
                md_cell(&row.room),
                md_cell(&row.component),
                md_cell(&row.criticality),
                md_cell(&row.diagram),
                row.status,
                md_cell(&row.gap)
            );
        } else {
            println!(
                "| {} | {} | {} | {} | {} |",
                md_cell(&row.component),
                md_cell(&row.criticality),
                md_cell(&row.diagram),
                row.status,
                md_cell(&row.gap)
            );
        }
    }
    if sorted.is_empty() {
        if include_room {
            println!("| none | none | none | none | none | none |");
        } else {
            println!("| none | none | none | none | none |");
        }
    }

    if include_room {
        print_visual_room_summary(rows);
        print_visual_gap_themes(rows);
        print_visual_sprint_queue(rows);
    }

    println!();
    println!("## Summary");
    println!();
    println!("| Status | Count |");
    println!("|---|---:|");
    println!("| Build-ready | {build_ready} |");
    println!("| Draft | {draft} |");
    println!("| Blocked | {blocked} |");
}

fn print_visual_room_summary(rows: &[VisualReportRow]) {
    let mut rooms: BTreeMap<&str, (u32, u32, u32, u32)> = BTreeMap::new();

    for row in rows {
        let entry = rooms.entry(&row.room).or_insert((0, 0, 0, 0));
        if row.status == "build-ready" {
            entry.0 += 1;
        } else if row.status == "draft" {
            entry.1 += 1;
        } else if row.status == "blocked" {
            entry.2 += 1;
        }
        if row.status != "build-ready" && is_c5(&row.criticality) {
            entry.3 += 1;
        }
    }

    println!();
    println!("## Room Summary");
    println!();
    println!("| Room | Build-ready | Draft | Blocked | Open C5 |");
    println!("|---|---:|---:|---:|---:|");
    if rooms.is_empty() {
        println!("| none | 0 | 0 | 0 | 0 |");
    }
    for (room, (ready, draft, blocked, open_c5)) in rooms {
        println!(
            "| {} | {ready} | {draft} | {blocked} | {open_c5} |",
            md_cell(room)
        );
    }
}

fn print_visual_gap_themes(rows: &[VisualReportRow]) {
    let mut themes: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for row in rows {
        if row.status == "build-ready" {
            continue;
        }
        let entry = themes.entry(&row.gap).or_insert((0, 0));
        entry.0 += 1;
        if is_c5(&row.criticality) {
            entry.1 += 1;
        }
    }

    let mut ranked: Vec<_> = themes.into_iter().collect();
    ranked.sort_by(|left, right| {
        right
            .1
             .1
            .cmp(&left.1 .1)
            .then_with(|| right.1 .0.cmp(&left.1 .0))
            .then_with(|| left.0.cmp(right.0))
    });

    println!();
    println!("## Repeated Gap Themes");
    println!();
    println!("| Gap / blocker | Components | C5 components |");
    println!("|---|---:|---:|");
    if ranked.is_empty() {
        println!("| none | 0 | 0 |");
    }
    for (gap, (count, c5_count)) in ranked.into_iter().take(10) {
        println!("| {} | {count} | {c5_count} |", md_cell(gap));
    }
}

fn print_visual_sprint_queue(rows: &[VisualReportRow]) {
    let mut themes: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for row in rows {
        if row.status == "build-ready" {
            continue;
        }
        let entry = themes.entry(&row.gap).or_insert((0, 0));
        entry.0 += 1;
        if is_c5(&row.criticality) {
            entry.1 += 1;
        }
    }

    let mut ranked: Vec<_> = themes.into_iter().collect();
    ranked.sort_by(|left, right| {
        right
            .1
             .1
            .cmp(&left.1 .1)
            .then_with(|| right.1 .0.cmp(&left.1 .0))
            .then_with(|| left.0.cmp(right.0))
    });

    println!();
    println!("## Recommended Sprint Queue");
    println!();
    println!("| Priority | Focus | Why now | Filter command |");
    println!("|---:|---|---|---|");
    if ranked.is_empty() {
        println!("| 0 | none | no open visual readiness items match this filter | `visuals --rooms rooms` |");
    }
    for (index, (gap, (count, c5_count))) in ranked.into_iter().take(5).enumerate() {
        let filter = sprint_gap_filter(gap);
        let command = if c5_count > 0 {
            format!("visuals --rooms rooms --open-c5 --gap \"{filter}\"")
        } else {
            format!("visuals --rooms rooms --status draft --gap \"{filter}\"")
        };
        let why = if c5_count > 0 {
            format!("{c5_count} C5 showstopper(s), {count} total component(s)")
        } else {
            format!("{count} component(s)")
        };
        println!(
            "| {} | {} | {} | `{}` |",
            index + 1,
            md_cell(gap),
            why,
            md_cell(&command)
        );
    }
}
