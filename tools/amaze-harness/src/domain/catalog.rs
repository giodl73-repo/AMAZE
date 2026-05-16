use std::collections::HashMap;

use crate::markdown::value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InventoryItem {
    pub(crate) id: String,
    pub(crate) item: String,
    pub(crate) common_sources: String,
    pub(crate) unit_cost_band: String,
    pub(crate) durability: String,
    pub(crate) typical_use: String,
}

impl InventoryItem {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            id: value(row, "ID"),
            item: value(row, "Item"),
            common_sources: value(row, "Common sources"),
            unit_cost_band: value(row, "Unit cost band"),
            durability: value(row, "Durability"),
            typical_use: value(row, "Typical use"),
        }
    }

    pub(crate) fn matches_query(&self, query: &str) -> bool {
        [
            &self.id,
            &self.item,
            &self.common_sources,
            &self.unit_cost_band,
            &self.durability,
            &self.typical_use,
        ]
        .iter()
        .any(|value| value.to_lowercase().contains(query))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BeatPackage {
    pub(crate) package_id: String,
    pub(crate) beat_package: String,
    pub(crate) use_when: String,
    pub(crate) techniques: String,
    pub(crate) default_devices: String,
    pub(crate) evidence_hooks: String,
}

impl BeatPackage {
    pub(crate) fn from_row(row: &HashMap<String, String>) -> Self {
        Self {
            package_id: value(row, "Package ID"),
            beat_package: value(row, "Beat package"),
            use_when: value(row, "Use when"),
            techniques: value(row, "Techniques"),
            default_devices: value(row, "Default devices"),
            evidence_hooks: value(row, "Evidence hooks"),
        }
    }

    pub(crate) fn matches_query(&self, query: &str) -> bool {
        [
            &self.package_id,
            &self.beat_package,
            &self.use_when,
            &self.techniques,
            &self.default_devices,
            &self.evidence_hooks,
        ]
        .iter()
        .any(|value| value.to_lowercase().contains(query))
    }
}
