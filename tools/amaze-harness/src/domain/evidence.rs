use std::collections::HashMap;

use crate::markdown::first_present;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvidenceProbe {
    pub(crate) kind: String,
    pub(crate) test_id: String,
    pub(crate) target: String,
    pub(crate) pressure: String,
    pub(crate) evidence: String,
    pub(crate) raw_status: String,
    pub(crate) blocker: String,
}

impl EvidenceProbe {
    pub(crate) fn from_bench_row(row: &HashMap<String, String>) -> Self {
        Self {
            kind: "bench".to_string(),
            test_id: first_present(row, &["Test ID", "Probe ID", "Run/Test ID"]),
            target: first_present(row, &["Device", "Beat/device", "Component"]),
            pressure: first_present(row, &["Risk band", "Pass type", "Crowd profile"]),
            evidence: first_present(row, &["Required observation", "Evidence needed"]),
            raw_status: first_present(row, &["Status", "Result"]),
            blocker: first_present(row, &["Promotion impact", "Follow-up", "Required change"]),
        }
    }

    pub(crate) fn from_admin_row(row: &HashMap<String, String>) -> Self {
        let failure = first_present(row, &["Failure simulated"]);
        let spare = first_present(row, &["Spare/bypass used"]);
        let target_minutes = first_present(row, &["Target replacement min"]);

        Self {
            kind: "admin".to_string(),
            test_id: first_present(row, &["Test ID", "Probe ID", "Run/Test ID"]),
            target: first_present(row, &["Component", "Beat/device", "Device"]),
            pressure: first_present(row, &["Criticality", "Failure simulated"]),
            evidence: format!("{failure}; {spare}; target {target_minutes} min"),
            raw_status: first_present(row, &["Result", "Status"]),
            blocker: first_present(row, &["Required change", "Post-run fix", "Follow-up"]),
        }
    }

    pub(crate) fn from_chaos_row(row: &HashMap<String, String>) -> Self {
        Self {
            kind: "chaos".to_string(),
            test_id: first_present(row, &["Probe ID", "Test ID"]),
            target: first_present(row, &["Beat/device", "Component", "Device"]),
            pressure: first_present(row, &["Probe action", "Failure simulated"]),
            evidence: first_present(row, &["Watch signal", "Expected recovery"]),
            raw_status: first_present(row, &["Status", "Result"]),
            blocker: first_present(row, &["Expected recovery", "Required change"]),
        }
    }
}
