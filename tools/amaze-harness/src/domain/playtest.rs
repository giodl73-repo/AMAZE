use std::collections::HashMap;

use crate::markdown::value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariabilityProfile {
    pub(crate) descriptor: String,
    pub(crate) issue_ids: String,
}

impl VariabilityProfile {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            descriptor: value(row, "Team/behavior"),
            issue_ids: value(row, "Reliability issue IDs"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChaosProbe {
    pub(crate) probe_id: String,
}

impl ChaosProbe {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            probe_id: value(row, "Probe ID"),
        }
    }
}
