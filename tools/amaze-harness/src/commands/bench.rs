use std::collections::BTreeMap;

use crate::cli::BenchArgs;
use crate::evidence::{
    bench_evidence_rows, blocker_filter_text, evidence_status_rank, filter_bench_rows,
    is_open_evidence_status, BenchEvidenceFilter, BenchEvidenceRow,
};
use crate::markdown::md_cell;
use crate::room::active_room_paths;

pub(crate) fn bench_evidence_report(args: BenchArgs) -> Result<(), String> {
    let filter = bench_filter(&args);
    if let Some(room) = args.room.as_ref() {
        let rows = filter_bench_rows(bench_evidence_rows(room)?, &filter);
        println!("# AMAZE Bench Evidence");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        println!("| Room | {} |", room.display());
        if args.open {
            println!("| Filter | open |");
        }
        if let Some(kind) = &args.kind {
            println!("| Kind filter | {kind} |");
        }
        if let Some(status) = &args.status {
            println!("| Status filter | {status} |");
        }
        if let Some(blocker) = &args.blocker {
            println!("| Blocker filter | {blocker} |");
        }
        if let Some(target) = &args.target {
            println!("| Target filter | {target} |");
        }
        println!();
        print_bench_rows(&rows, false);
        return Ok(());
    }

    let rooms_root = args
        .rooms
        .as_ref()
        .ok_or_else(|| "bench requires --room <path> or --rooms <rooms-root>".to_string())?;
    let mut rows = Vec::new();
    for room in active_room_paths(rooms_root)? {
        rows.extend(bench_evidence_rows(&room)?);
    }
    let rows = filter_bench_rows(rows, &filter);

    println!("# AMAZE Bench Evidence Portfolio");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Rooms root | {} |", rooms_root.display());
    if args.open {
        println!("| Filter | open |");
    }
    if let Some(kind) = &args.kind {
        println!("| Kind filter | {kind} |");
    }
    if let Some(status) = &args.status {
        println!("| Status filter | {status} |");
    }
    if let Some(blocker) = &args.blocker {
        println!("| Blocker filter | {blocker} |");
    }
    if let Some(target) = &args.target {
        println!("| Target filter | {target} |");
    }
    println!();
    print_bench_rows(&rows, true);

    Ok(())
}

fn bench_filter(args: &BenchArgs) -> BenchEvidenceFilter<'_> {
    BenchEvidenceFilter {
        open: args.open,
        kind: args.kind.as_deref(),
        status: args.status.as_deref(),
        blocker: args.blocker.as_deref(),
        target: args.target.as_deref(),
    }
}

fn print_bench_rows(rows: &[BenchEvidenceRow], include_room: bool) {
    let mut passed = 0;
    let mut open = 0;
    let mut sorted: Vec<&BenchEvidenceRow> = rows.iter().collect();
    sorted.sort_by(|left, right| {
        evidence_status_rank(&left.status)
            .cmp(&evidence_status_rank(&right.status))
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.room.cmp(&right.room))
            .then_with(|| left.test_id.cmp(&right.test_id))
    });

    println!("## Evidence Queue");
    println!();
    if include_room {
        println!("| Room | Kind | Test ID | Target | Pressure | Evidence needed | Status | Blocker / next move |");
        println!("|---|---|---|---|---|---|---|---|");
    } else {
        println!("| Kind | Test ID | Target | Pressure | Evidence needed | Status | Blocker / next move |");
        println!("|---|---|---|---|---|---|---|");
    }

    for row in &sorted {
        if is_open_evidence_status(&row.status) {
            open += 1;
        } else {
            passed += 1;
        }

        if include_room {
            println!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                md_cell(&row.room),
                row.kind,
                md_cell(&row.test_id),
                md_cell(&row.target),
                md_cell(&row.pressure),
                md_cell(&row.evidence),
                row.status,
                md_cell(&row.blocker)
            );
        } else {
            println!(
                "| {} | {} | {} | {} | {} | {} | {} |",
                row.kind,
                md_cell(&row.test_id),
                md_cell(&row.target),
                md_cell(&row.pressure),
                md_cell(&row.evidence),
                row.status,
                md_cell(&row.blocker)
            );
        }
    }
    if sorted.is_empty() {
        if include_room {
            println!("| none | none | none | none | none | none | none | none |");
        } else {
            println!("| none | none | none | none | none | none | none |");
        }
    }

    if include_room {
        print_bench_room_summary(rows);
        print_bench_kind_summary(rows);
        print_bench_sprint_queue(rows);
    }

    println!();
    println!("## Summary");
    println!();
    println!("| Status | Count |");
    println!("|---|---:|");
    println!("| Passed | {passed} |");
    println!("| Open | {open} |");
}

fn print_bench_room_summary(rows: &[BenchEvidenceRow]) {
    let mut rooms: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for row in rows {
        let entry = rooms.entry(&row.room).or_insert((0, 0));
        if is_open_evidence_status(&row.status) {
            entry.1 += 1;
        } else {
            entry.0 += 1;
        }
    }

    println!();
    println!("## Room Summary");
    println!();
    println!("| Room | Passed | Open |");
    println!("|---|---:|---:|");
    if rooms.is_empty() {
        println!("| none | 0 | 0 |");
    }
    for (room, (passed, open)) in rooms {
        println!("| {} | {passed} | {open} |", md_cell(room));
    }
}

fn print_bench_kind_summary(rows: &[BenchEvidenceRow]) {
    let mut kinds: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for row in rows {
        let entry = kinds.entry(&row.kind).or_insert((0, 0));
        if is_open_evidence_status(&row.status) {
            entry.1 += 1;
        } else {
            entry.0 += 1;
        }
    }

    println!();
    println!("## Kind Summary");
    println!();
    println!("| Kind | Passed | Open |");
    println!("|---|---:|---:|");
    if kinds.is_empty() {
        println!("| none | 0 | 0 |");
    }
    for (kind, (passed, open)) in kinds {
        println!("| {kind} | {passed} | {open} |");
    }
}

fn print_bench_sprint_queue(rows: &[BenchEvidenceRow]) {
    let mut themes: BTreeMap<&str, u32> = BTreeMap::new();
    for row in rows {
        if is_open_evidence_status(&row.status) {
            *themes.entry(&row.blocker).or_insert(0) += 1;
        }
    }
    let mut ranked: Vec<_> = themes.into_iter().collect();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));

    println!();
    println!("## Recommended Evidence Sprint");
    println!();
    println!("| Priority | Focus | Open items | Filter command |");
    println!("|---:|---|---:|---|");
    if ranked.is_empty() {
        println!("| 0 | none | 0 | `bench --rooms rooms` |");
    }
    for (index, (blocker, count)) in ranked.into_iter().take(5).enumerate() {
        let kind = if blocker.to_lowercase().contains("reset")
            || blocker.to_lowercase().contains("pouch")
            || blocker.to_lowercase().contains("stage")
        {
            "admin"
        } else {
            "bench"
        };
        let filter = blocker_filter_text(blocker);
        println!(
            "| {} | {} | {} | `bench --rooms rooms --open --kind {kind} --blocker \"{filter}\"` |",
            index + 1,
            md_cell(blocker),
            count
        );
    }
}
