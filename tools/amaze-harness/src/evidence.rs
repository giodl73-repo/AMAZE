use std::path::Path;

use crate::domain::EvidenceProbe;
use crate::room::read_playtest_doc;

#[derive(Debug)]
pub(crate) struct BenchEvidenceRow {
    pub(crate) room: String,
    pub(crate) kind: String,
    pub(crate) test_id: String,
    pub(crate) target: String,
    pub(crate) pressure: String,
    pub(crate) evidence: String,
    pub(crate) status: String,
    pub(crate) blocker: String,
}

#[derive(Debug)]
pub(crate) struct BenchEvidenceFilter<'a> {
    pub(crate) open: bool,
    pub(crate) kind: Option<&'a str>,
    pub(crate) status: Option<&'a str>,
    pub(crate) blocker: Option<&'a str>,
    pub(crate) target: Option<&'a str>,
}

pub(crate) fn filter_bench_rows(
    rows: Vec<BenchEvidenceRow>,
    filter: &BenchEvidenceFilter<'_>,
) -> Vec<BenchEvidenceRow> {
    rows.into_iter()
        .filter(|row| !filter.open || is_open_evidence_status(&row.status))
        .filter(|row| filter.kind.is_none_or(|kind| row.kind == kind))
        .filter(|row| filter.status.is_none_or(|status| row.status == status))
        .filter(|row| {
            filter
                .blocker
                .is_none_or(|blocker| row.blocker.to_lowercase().contains(blocker))
        })
        .filter(|row| {
            filter.target.is_none_or(|target| {
                row.target.to_lowercase().contains(target)
                    || row.test_id.to_lowercase().contains(target)
            })
        })
        .collect()
}

pub(crate) fn bench_evidence_rows(room: &Path) -> Result<Vec<BenchEvidenceRow>, String> {
    let room_name = room.display().to_string();
    let playtest_doc = read_playtest_doc(room)?;
    let rows = playtest_doc
        .into_evidence_probes()
        .into_iter()
        .map(|probe| evidence_report_row(&room_name, probe))
        .collect();

    Ok(rows)
}

fn evidence_report_row(room: &str, probe: EvidenceProbe) -> BenchEvidenceRow {
    BenchEvidenceRow {
        room: room.to_string(),
        kind: probe.kind,
        test_id: probe.test_id,
        target: probe.target,
        pressure: probe.pressure,
        evidence: probe.evidence,
        status: normalize_evidence_status(&probe.raw_status),
        blocker: probe.blocker,
    }
}

pub(crate) fn normalize_evidence_status(raw: &str) -> String {
    let lower = raw.trim().to_lowercase();
    if lower.is_empty() || matches!(lower.as_str(), "tbd" | "none" | "n/a" | "na") {
        "pending".to_string()
    } else if lower.contains("not run") || lower.contains("not-run") {
        "not-run".to_string()
    } else if lower.contains("pending") {
        "pending".to_string()
    } else if lower.contains("block") {
        "blocked".to_string()
    } else if lower.contains("fail") {
        "failed".to_string()
    } else if lower.contains("pass") || lower.contains("done") || lower.contains("complete") {
        "passed".to_string()
    } else {
        lower
    }
}

pub(crate) fn is_open_evidence_status(status: &str) -> bool {
    !matches!(status, "passed" | "complete" | "done")
}

pub(crate) fn blocker_filter_text(blocker: &str) -> String {
    for prefix in ["blocks ", "stage ", "label ", "verify ", "prove "] {
        if let Some(rest) = blocker.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    blocker.to_string()
}

pub(crate) fn evidence_status_rank(status: &str) -> u8 {
    match status {
        "blocked" | "failed" => 0,
        "not-run" => 1,
        "pending" => 2,
        "passed" => 3,
        _ => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        blocker_filter_text, evidence_report_row, filter_bench_rows, normalize_evidence_status,
        BenchEvidenceFilter, BenchEvidenceRow,
    };
    use crate::domain::PlaytestDoc;

    #[test]
    fn normalizes_evidence_statuses() {
        assert_eq!(normalize_evidence_status("Not run"), "not-run");
        assert_eq!(normalize_evidence_status("pending"), "pending");
        assert_eq!(normalize_evidence_status("Passed"), "passed");
        assert_eq!(normalize_evidence_status("TBD"), "pending");
    }

    #[test]
    fn trims_common_blocker_filter_prefixes() {
        assert_eq!(blocker_filter_text("blocks P2 promotion"), "P2 promotion");
        assert_eq!(blocker_filter_text("stage P2 pouch"), "P2 pouch");
    }

    #[test]
    fn extracts_bench_and_admin_rows() {
        let markdown = "\
# Playtest

## Device bench tests

| Test ID | Device | Risk band | Required observation | Status | Promotion impact |
|---|---|---|---|---|---|
| BT-1 | `DEV-SWITCH-001` | Orange | no unsafe state | pending | blocks P2 promotion |

## Admin replacement drills

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| AR-1 | switch | C4 required | switch fails | manual card | 2 | TBD | Not run | stage card |
";
        let probes = PlaytestDoc::from_markdown(markdown).into_evidence_probes();
        let bench_test_id = probes
            .iter()
            .find(|probe| probe.kind == "bench")
            .expect("bench evidence")
            .test_id
            .clone();
        let admin_probe = probes
            .into_iter()
            .find(|probe| probe.kind == "admin")
            .expect("admin evidence");
        let admin_row = evidence_report_row("room", admin_probe);

        assert_eq!(bench_test_id, "BT-1");
        assert_eq!(admin_row.status, "not-run");
        assert_eq!(
            admin_row.evidence,
            "switch fails; manual card; target 2 min"
        );
    }

    #[test]
    fn filters_open_evidence_by_kind_target_and_blocker() {
        let rows = vec![
            row("bench", "BT-1", "socket", "pending", "blocks P2 promotion"),
            row("admin", "AR-1", "socket", "not-run", "stage pouch"),
            row("bench", "BT-2", "lamp", "passed", "none"),
        ];
        let filter = BenchEvidenceFilter {
            open: true,
            kind: Some("bench"),
            status: None,
            blocker: Some("p2"),
            target: Some("socket"),
        };

        let filtered = filter_bench_rows(rows, &filter);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].test_id, "BT-1");
    }

    fn row(
        kind: &str,
        test_id: &str,
        target: &str,
        status: &str,
        blocker: &str,
    ) -> BenchEvidenceRow {
        BenchEvidenceRow {
            room: "room".to_string(),
            kind: kind.to_string(),
            test_id: test_id.to_string(),
            target: target.to_string(),
            pressure: "pressure".to_string(),
            evidence: "evidence".to_string(),
            status: status.to_string(),
            blocker: blocker.to_string(),
        }
    }
}
