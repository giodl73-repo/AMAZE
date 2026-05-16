use std::collections::HashMap;

use crate::markdown::value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NpcCharacter {
    pub(crate) name: String,
    pub(crate) function: String,
    pub(crate) trigger: String,
    pub(crate) delivery_mode: String,
    pub(crate) sample_line: String,
    pub(crate) limits: String,
    pub(crate) reset_ops_burden: String,
}

impl NpcCharacter {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            name: value(row, "NPC/voice"),
            function: value(row, "Function"),
            trigger: value(row, "Trigger"),
            delivery_mode: value(row, "Delivery mode"),
            sample_line: value(row, "Sample line"),
            limits: value(row, "Limits"),
            reset_ops_burden: value(row, "Reset/ops burden"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperatorRotation {
    pub(crate) scope: String,
    pub(crate) coverage_model: String,
    pub(crate) max_rooms: String,
    pub(crate) safe_when: String,
    pub(crate) unsafe_when: String,
    pub(crate) handoff_signal: String,
    pub(crate) dedicated_staff_trigger: String,
}

impl OperatorRotation {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            scope: value(row, "Scope"),
            coverage_model: value(row, "Coverage model"),
            max_rooms: value(row, "Max rooms"),
            safe_when: value(row, "Safe when"),
            unsafe_when: value(row, "Unsafe when"),
            handoff_signal: value(row, "Handoff signal"),
            dedicated_staff_trigger: value(row, "Dedicated staff trigger"),
        }
    }
}
