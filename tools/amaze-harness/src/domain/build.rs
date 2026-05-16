use std::collections::HashMap;

use crate::markdown::{first_present, parse_minutes_field, value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Component {
    pub(crate) name: String,
    pub(crate) inventory_id: String,
    pub(crate) beat_ids: String,
    pub(crate) failure_mode: String,
    pub(crate) criticality: String,
    pub(crate) durability_class: String,
    pub(crate) source_tier: String,
    pub(crate) visual_reference: String,
    pub(crate) build_minutes: Option<u32>,
    pub(crate) replacement_minutes: Option<u32>,
    pub(crate) spare_qty: Option<u32>,
    pub(crate) bypass: String,
    pub(crate) admin_recovery: String,
}

impl Component {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            name: value(row, "Component"),
            inventory_id: value(row, "Inventory ID"),
            beat_ids: value(row, "Beat IDs"),
            failure_mode: value(row, "Failure mode"),
            criticality: value(row, "Criticality"),
            durability_class: value(row, "Durability class"),
            source_tier: value(row, "Source tier"),
            visual_reference: first_present(
                row,
                &[
                    "Visual reference",
                    "Visual Reference",
                    "Part diagram",
                    "Diagram",
                ],
            ),
            build_minutes: parse_minutes_field(
                row,
                &[
                    "Build time",
                    "Build hours",
                    "Prototype build time",
                    "Production build time",
                ],
            ),
            replacement_minutes: parse_minutes_field(
                row,
                &[
                    "Replacement min",
                    "Replace min",
                    "Admin replace min",
                    "Recovery min",
                ],
            ),
            spare_qty: parse_minutes_field(row, &["Spare qty", "Spare Quantity", "Spares"]),
            bypass: value(row, "Bypass"),
            admin_recovery: value(row, "Admin recovery"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PartDiagram {
    pub(crate) scope: String,
    pub(crate) diagram: String,
    pub(crate) proof: String,
    pub(crate) build_readiness_gap: String,
}

impl PartDiagram {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            scope: first_present(
                row,
                &["Beat/device", "Beat", "Device", "Component", "Applies to"],
            ),
            diagram: value(row, "Diagram"),
            proof: value(row, "What it proves"),
            build_readiness_gap: first_present(
                row,
                &[
                    "Missing before build readiness",
                    "Missing before build",
                    "Build readiness gap",
                ],
            ),
        }
    }
}
