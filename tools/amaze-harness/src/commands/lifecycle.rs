use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cli::LifecycleArgs;
use crate::evidence::{bench_evidence_rows, is_open_evidence_status};
use crate::markdown::md_cell;
use crate::room::{active_room_paths, read_room_file, room_contract_issues, BRIEF_FILE};
use crate::rules::visual::{is_c5, visual_report_rows};

#[derive(Debug)]
struct LifecycleRoom {
    room: PathBuf,
    state: String,
    next_gate: String,
    contract_issues: usize,
    open_c5_visuals: usize,
    blocked_visuals: usize,
    open_bench_items: usize,
}

pub(crate) fn lifecycle_report(args: LifecycleArgs) -> Result<(), String> {
    let reports = lifecycle_targets(&args)?
        .iter()
        .map(|room| lifecycle_room(room))
        .collect::<Result<Vec<_>, _>>()?;

    if reports.len() == 1 {
        println!("# AMAZE Room Lifecycle");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        println!("| Room | {} |", reports[0].room.display());
        println!();
        print_lifecycle_rows(&reports, false);
    } else {
        println!("# AMAZE Lifecycle Portfolio");
        println!();
        println!("| Field | Value |");
        println!("|---|---|");
        let rooms_root = args
            .rooms
            .as_ref()
            .ok_or_else(|| "lifecycle portfolio requires --rooms <rooms-root>".to_string())?;
        println!("| Rooms root | {} |", rooms_root.display());
        println!("| Rooms reported | {} |", reports.len());
        println!();
        print_lifecycle_rows(&reports, true);
        print_state_summary(&reports);
        print_gate_sprint(&reports);
    }

    Ok(())
}

fn lifecycle_targets(args: &LifecycleArgs) -> Result<Vec<PathBuf>, String> {
    if let Some(room) = &args.room {
        return Ok(vec![room.clone()]);
    }

    let rooms_root = args
        .rooms
        .as_deref()
        .ok_or_else(|| "lifecycle requires --room <path> or --rooms <rooms-root>".to_string())?;
    active_room_paths(rooms_root)
}

fn lifecycle_room(room: &Path) -> Result<LifecycleRoom, String> {
    let brief = read_room_file(room, BRIEF_FILE)?;
    let visuals = visual_report_rows(room)?;
    let open_c5_visuals = visuals
        .iter()
        .filter(|row| row.status != "build-ready" && is_c5(&row.criticality))
        .count();
    let blocked_visuals = visuals.iter().filter(|row| row.status == "blocked").count();
    let open_bench_items = bench_evidence_rows(room)?
        .iter()
        .filter(|row| is_open_evidence_status(&row.status))
        .count();

    Ok(LifecycleRoom {
        room: room.to_path_buf(),
        state: status_value(&brief, "Lifecycle state").unwrap_or_else(|| "Unknown".to_string()),
        next_gate: status_value(&brief, "Next gate").unwrap_or_else(|| "not declared".to_string()),
        contract_issues: room_contract_issues(room)?.len(),
        open_c5_visuals,
        blocked_visuals,
        open_bench_items,
    })
}

fn print_lifecycle_rows(reports: &[LifecycleRoom], include_room: bool) {
    println!("## Gate Status");
    println!();
    if include_room {
        println!("| Room | State | Declared next gate | Contract | Open C5 visuals | Blocked visuals | Open bench | Recommendation |");
        println!("|---|---|---|---:|---:|---:|---:|---|");
    } else {
        println!("| State | Declared next gate | Contract | Open C5 visuals | Blocked visuals | Open bench | Recommendation |");
        println!("|---|---|---:|---:|---:|---:|---|");
    }

    if reports.is_empty() {
        if include_room {
            println!("| none | none | none | 0 | 0 | 0 | 0 | no rooms reported |");
        } else {
            println!("| none | none | 0 | 0 | 0 | 0 | no room reported |");
        }
    }

    for report in reports {
        let recommendation = gate_recommendation(report);
        if include_room {
            println!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                md_cell(&report.room.display().to_string()),
                md_cell(&report.state),
                md_cell(&report.next_gate),
                report.contract_issues,
                report.open_c5_visuals,
                report.blocked_visuals,
                report.open_bench_items,
                md_cell(recommendation)
            );
        } else {
            println!(
                "| {} | {} | {} | {} | {} | {} | {} |",
                md_cell(&report.state),
                md_cell(&report.next_gate),
                report.contract_issues,
                report.open_c5_visuals,
                report.blocked_visuals,
                report.open_bench_items,
                md_cell(recommendation)
            );
        }
    }
    println!();
}

fn print_state_summary(reports: &[LifecycleRoom]) {
    let mut states: BTreeMap<&str, u32> = BTreeMap::new();
    for report in reports {
        *states.entry(&report.state).or_default() += 1;
    }

    println!("## Lifecycle State Summary");
    println!();
    println!("| State | Rooms |");
    println!("|---|---:|");
    if states.is_empty() {
        println!("| none | 0 |");
    }
    for (state, count) in states {
        println!("| {} | {count} |", md_cell(state));
    }
    println!();
}

fn print_gate_sprint(reports: &[LifecycleRoom]) {
    let mut contract = 0;
    let mut design_simulation = 0;
    let mut visual = 0;
    let mut bench = 0;
    let mut review = 0;

    for report in reports {
        if report.contract_issues > 0 {
            contract += 1;
        } else if is_early_design_state(report) {
            design_simulation += 1;
        } else if report.open_c5_visuals > 0 || report.blocked_visuals > 0 {
            visual += 1;
        } else if report.open_bench_items > 0 {
            bench += 1;
        } else {
            review += 1;
        }
    }

    println!("## Recommended Gate Sprint");
    println!();
    println!("| Priority | Focus | Rooms | Command |");
    println!("|---:|---|---:|---|");
    let rows = [
        (
            "contract hygiene",
            contract,
            "lint --rooms rooms && check --room <room>",
        ),
        (
            "graph/timing/simulation gate",
            design_simulation,
            "matrix --room <room> && simulate --room <room>",
        ),
        (
            "C4-C5 visual blockers",
            visual,
            "visuals --rooms rooms --open-c5",
        ),
        (
            "bench/admin/chaos evidence",
            bench,
            "bench --rooms rooms --open",
        ),
        ("next declared review", review, "lifecycle --room <room>"),
    ];
    for (index, (focus, count, command)) in rows.into_iter().enumerate() {
        println!("| {} | {} | {} | `{}` |", index + 1, focus, count, command);
    }
}

fn gate_recommendation(report: &LifecycleRoom) -> &'static str {
    if report.contract_issues > 0 {
        "fix room contract before promotion"
    } else if report.state.eq_ignore_ascii_case("Seed") {
        "complete graph/timing/tabletop gate before build evidence"
    } else if report.state.eq_ignore_ascii_case("Graphed") {
        "run matrix and tabletop simulations"
    } else if report.open_c5_visuals > 0 {
        "resolve C5 visual blockers before build-candidate review"
    } else if report.blocked_visuals > 0 {
        "clear blocked visual rows before build planning"
    } else if report.open_bench_items > 0 {
        "run bench/admin/chaos evidence before playtest-ready review"
    } else {
        "ready for next declared gate review"
    }
}

fn is_early_design_state(report: &LifecycleRoom) -> bool {
    report.state.eq_ignore_ascii_case("Seed")
        || report.state.eq_ignore_ascii_case("Briefed")
        || report.state.eq_ignore_ascii_case("Mapped")
        || report.state.eq_ignore_ascii_case("Graphed")
}

fn status_value(markdown: &str, field: &str) -> Option<String> {
    let prefix = format!("- {field}:");
    markdown.lines().find_map(|line| {
        line.trim()
            .strip_prefix(&prefix)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::{gate_recommendation, is_early_design_state, status_value, LifecycleRoom};
    use std::path::PathBuf;

    #[test]
    fn status_value_reads_brief_status_field() {
        let brief = "- Lifecycle state: Graphed\n- Next gate: simulated\n";
        assert_eq!(
            status_value(brief, "Lifecycle state").as_deref(),
            Some("Graphed")
        );
        assert_eq!(
            status_value(brief, "Next gate").as_deref(),
            Some("simulated")
        );
    }

    #[test]
    fn gate_recommendation_prioritizes_blockers() {
        let report = LifecycleRoom {
            room: PathBuf::from("rooms\\demo"),
            state: "Simulated".to_string(),
            next_gate: "build-candidate".to_string(),
            contract_issues: 0,
            open_c5_visuals: 1,
            blocked_visuals: 0,
            open_bench_items: 3,
        };
        assert_eq!(
            gate_recommendation(&report),
            "resolve C5 visual blockers before build-candidate review"
        );
    }

    #[test]
    fn early_design_states_wait_on_simulation_before_bench() {
        let report = LifecycleRoom {
            room: PathBuf::from("rooms\\demo"),
            state: "Graphed".to_string(),
            next_gate: "simulated".to_string(),
            contract_issues: 0,
            open_c5_visuals: 0,
            blocked_visuals: 0,
            open_bench_items: 3,
        };
        assert!(is_early_design_state(&report));
        assert_eq!(
            gate_recommendation(&report),
            "run matrix and tabletop simulations"
        );
    }
}
