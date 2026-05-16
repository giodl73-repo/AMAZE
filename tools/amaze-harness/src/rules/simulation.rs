use std::collections::HashMap;

use crate::domain::{ChaosProbe, Component, TeamProbe, VariabilityProfile};
use crate::sim::{sample_between, Lcg};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BuildTimePressure {
    pub(crate) average_hours: f64,
    pub(crate) min_hours: u32,
    pub(crate) max_hours: u32,
    pub(crate) bottlenecks: Vec<(String, u32)>,
}

pub(crate) fn choose_component_event<'a>(
    components: Option<&'a [Component]>,
    beat_name: &str,
    chance: u32,
    rng: &mut Lcg,
) -> Option<&'a Component> {
    if !rng.percent(chance) {
        return None;
    }
    let components = components?;
    let beat_id = beat_name
        .split_whitespace()
        .next()
        .unwrap_or(beat_name)
        .to_lowercase();
    let candidates: Vec<&Component> = components
        .iter()
        .filter(|component| {
            let beats = component.beat_ids.to_lowercase();
            beats.contains(&beat_id)
                || beats.contains("all")
                || beat_id
                    .split('-')
                    .next()
                    .is_some_and(|prefix| beats.contains(prefix))
        })
        .collect();
    if candidates.is_empty() {
        None
    } else {
        let index = rng.next_u32(candidates.len() as u32) as usize;
        candidates.get(index).copied()
    }
}

pub(crate) fn component_event_chance(risk: &str, variability: Option<&VariabilityProfile>) -> u32 {
    let mut chance = 3;
    let issue_text = variability
        .map(|profile| profile.issue_ids.as_str())
        .unwrap_or_default();
    let haystack = format!("{} {}", risk.to_lowercase(), issue_text.to_lowercase());
    for keyword in [
        "rel-loss",
        "rel-jam",
        "rel-force",
        "rel-reset",
        "loose",
        "jam",
        "force",
    ] {
        if haystack.contains(keyword) {
            chance += 5;
        }
    }
    chance.min(35)
}

pub(crate) fn component_failure_mode(
    component: &Component,
    beat_risk: &str,
    rng: &mut Lcg,
) -> String {
    let explicit = component.failure_mode.as_str();
    if explicit != "TBD" {
        return first_failure_word(explicit);
    }
    let haystack = beat_risk.to_lowercase();
    if haystack.contains("loss") || haystack.contains("pocket") || rng.percent(35) {
        "not-found".to_string()
    } else if haystack.contains("jam") || rng.percent(50) {
        "jammed".to_string()
    } else {
        "broken".to_string()
    }
}

fn first_failure_word(text: &str) -> String {
    let lower = text.to_lowercase();
    if lower.contains("missing") || lower.contains("loss") || lower.contains("lost") {
        "not-found".to_string()
    } else if lower.contains("jam") || lower.contains("stick") {
        "jammed".to_string()
    } else if lower.contains("break") || lower.contains("crack") || lower.contains("worn") {
        "broken".to_string()
    } else {
        text.split([',', ';'])
            .next()
            .unwrap_or("failure")
            .trim()
            .to_string()
    }
}

pub(crate) fn replacement_minutes(component: &Component) -> u32 {
    component.replacement_minutes.unwrap_or_else(|| {
        let criticality = component.criticality.to_lowercase();
        if criticality.contains("show") || criticality.contains("c5") {
            4
        } else if criticality.contains("critical") || criticality.contains("c4") {
            3
        } else {
            2
        }
    })
}

pub(crate) fn replacement_window(target: u32, slow: u32) -> u32 {
    (slow.saturating_sub(target) + 1).max(2)
}

pub(crate) fn build_minutes(component: &Component) -> u32 {
    component.build_minutes.unwrap_or(0)
}

pub(crate) fn build_variance_minutes(component: &Component) -> u32 {
    let base = build_minutes(component);
    let criticality = component.criticality.to_lowercase();
    let durability = component.durability_class.to_lowercase();
    let source = component.source_tier.to_lowercase();
    let mut variance = base / 4;
    if criticality.contains("c4") || criticality.contains("c5") {
        variance += base / 4 + 1;
    }
    if durability.contains("unknown") || durability == "d" || durability.starts_with("d/") {
        variance += base / 3 + 1;
    }
    if source.contains("custom") || source.contains("fabrication") {
        variance += base / 2 + 1;
    }
    variance
}

pub(crate) fn spare_qty(component: &Component) -> u32 {
    component.spare_qty.unwrap_or(0)
}

pub(crate) fn has_recovery_path(component: &Component) -> bool {
    let bypass = component.bypass.to_lowercase();
    let recovery = component.admin_recovery.to_lowercase();
    [bypass, recovery].iter().any(|text| {
        !text.is_empty()
            && text != "tbd"
            && text != "none"
            && (text.contains("staff")
                || text.contains("operator")
                || text.contains("duplicate")
                || text.contains("bypass")
                || text.contains("manual")
                || text.contains("release")
                || text.contains("spare")
                || text.contains("replace"))
    })
}

pub(crate) fn simulate_build_pressure(
    components: &[Component],
    runs: u32,
    seed: u64,
) -> Option<BuildTimePressure> {
    let build_items: Vec<&Component> = components
        .iter()
        .filter(|row| build_minutes(row) > 0)
        .collect();
    if build_items.is_empty() {
        return None;
    }

    let mut rng = Lcg::new(seed ^ 0xA11CE_B01D);
    let mut total_hours = 0;
    let mut min_hours = u32::MAX;
    let mut max_hours = 0;
    let mut bottlenecks: HashMap<String, u32> = HashMap::new();

    for _ in 0..runs {
        let mut run_hours = 0;
        let mut slowest_name = String::new();
        let mut slowest_hours = 0;

        for item in &build_items {
            let base = build_minutes(item);
            let variance = build_variance_minutes(item);
            let sampled = if variance == 0 {
                base
            } else {
                sample_between(&mut rng, base, base + variance)
            };
            run_hours += sampled;
            if sampled > slowest_hours {
                slowest_hours = sampled;
                slowest_name = format!("{} ({})", item.name, item.inventory_id);
            }
        }

        total_hours += run_hours;
        min_hours = min_hours.min(run_hours);
        max_hours = max_hours.max(run_hours);
        *bottlenecks.entry(slowest_name).or_insert(0) += 1;
    }

    let mut ranked: Vec<_> = bottlenecks.into_iter().collect();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

    Some(BuildTimePressure {
        average_hours: total_hours as f64 / runs as f64,
        min_hours,
        max_hours,
        bottlenecks: ranked,
    })
}

pub(crate) fn choose_team(team_probes: Option<&[TeamProbe]>, rng: &mut Lcg) -> String {
    let Some(probes) = team_probes else {
        return "Unspecified team".to_string();
    };
    if probes.is_empty() {
        return "Unspecified team".to_string();
    }
    let index = rng.next_u32(probes.len() as u32) as usize;
    probes[index].archetype.clone()
}

pub(crate) fn choose_chaos<'a>(
    chaos_probes: Option<&'a [ChaosProbe]>,
    rng: &mut Lcg,
) -> Option<&'a ChaosProbe> {
    let probes = chaos_probes?;
    if probes.is_empty() {
        return None;
    }
    let index = rng.next_u32(probes.len() as u32) as usize;
    probes.get(index)
}

pub(crate) fn choose_variability<'a>(
    variability_profiles: Option<&'a [VariabilityProfile]>,
    team: &str,
    behavior: &str,
    rng: &mut Lcg,
) -> Option<&'a VariabilityProfile> {
    let profiles = variability_profiles?;
    if profiles.is_empty() {
        return None;
    }
    let team_lower = team.to_lowercase();
    let behavior_lower = behavior.to_lowercase();
    let matches: Vec<&VariabilityProfile> = profiles
        .iter()
        .filter(|profile| {
            let descriptor = profile.descriptor.to_lowercase();
            descriptor
                .split(|ch: char| !ch.is_alphanumeric())
                .any(|part| {
                    part.len() > 3 && (team_lower.contains(part) || behavior_lower.contains(part))
                })
        })
        .collect();
    if matches.is_empty() {
        let index = rng.next_u32(profiles.len() as u32) as usize;
        profiles.get(index)
    } else {
        let index = rng.next_u32(matches.len() as u32) as usize;
        matches.get(index).copied()
    }
}

pub(crate) fn event_chance(team: &str, behavior: &str, behavior_probe: &str, risk: &str) -> u32 {
    let haystack = format!(
        "{} {} {} {}",
        team.to_lowercase(),
        behavior.to_lowercase(),
        behavior_probe.to_lowercase(),
        risk.to_lowercase()
    );
    let mut chance = 10;
    for keyword in [
        "chaotic", "speed", "force", "guess", "random", "audio", "access", "fighting", "confused",
        "reset", "jam", "false",
    ] {
        if haystack.contains(keyword) {
            chance += 8;
        }
    }
    chance.min(70)
}

pub(crate) fn first_issue_id(text: &str) -> Option<String> {
    text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-'))
        .find(|part| part.starts_with("REL-") || part.starts_with("CHAOS-"))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::{components_from_table, Component, VariabilityProfile};
    use crate::markdown::Table;
    use crate::sim::Lcg;

    use super::{
        build_variance_minutes, choose_component_event, component_event_chance,
        component_failure_mode, first_issue_id, has_recovery_path, replacement_minutes,
        simulate_build_pressure, spare_qty,
    };

    #[test]
    fn extracts_first_issue_id() {
        assert_eq!(
            first_issue_id("force risk REL-FORCE, REL-FP"),
            Some("REL-FORCE".to_string())
        );
        assert_eq!(first_issue_id("no issue"), None);
    }

    #[test]
    fn simulates_build_pressure_with_ranked_bottlenecks() {
        let table = Table {
            rows: vec![
                row(&[
                    ("Component", "fast prop"),
                    ("Inventory ID", "PROP-001"),
                    ("Build time", "1"),
                ]),
                row(&[
                    ("Component", "custom cabinet"),
                    ("Inventory ID", "CAB-001"),
                    ("Build time", "5"),
                    ("Source tier", "custom"),
                ]),
            ],
        };

        let components = components_from_table(&table);
        let pressure = simulate_build_pressure(&components, 5, 7).expect("build pressure");

        assert!(pressure.average_hours >= 6.0);
        assert_eq!(pressure.bottlenecks[0].0, "custom cabinet (CAB-001)");
    }

    #[test]
    fn component_recovery_rules_use_typed_component_fields() {
        let component = component(&[
            ("Component", "showstopper latch"),
            ("Inventory ID", "MECH-LOCK-001"),
            ("Beat IDs", "P2"),
            ("Criticality", "C5 showstopper"),
            ("Failure mode", "missing key"),
            ("Spare qty", "2"),
            ("Admin recovery", "staff duplicate key"),
            ("Build time", "4"),
            ("Source tier", "custom fabrication"),
        ]);

        assert_eq!(replacement_minutes(&component), 4);
        assert_eq!(spare_qty(&component), 2);
        assert!(has_recovery_path(&component));
        assert!(build_variance_minutes(&component) > 4);
        assert_eq!(
            component_failure_mode(&component, "REL-LOSS", &mut Lcg::new(1)),
            "not-found"
        );
    }

    #[test]
    fn manual_release_counts_as_recovery_path() {
        let component = component(&[
            ("Component", "orrery vault"),
            ("Inventory ID", "CUSTOM-SCENIC"),
            ("Bypass", "manual vault card"),
            ("Admin recovery", "rear hatch/manual release"),
        ]);

        assert!(has_recovery_path(&component));
    }

    #[test]
    fn component_event_selection_matches_beat_ids() {
        let components = vec![
            component(&[
                ("Component", "unrelated"),
                ("Inventory ID", "A"),
                ("Beat IDs", "P9"),
            ]),
            component(&[
                ("Component", "socket"),
                ("Inventory ID", "B"),
                ("Beat IDs", "P2-P3"),
            ]),
        ];

        let selected =
            choose_component_event(Some(&components), "P2 socket proof", 100, &mut Lcg::new(2))
                .expect("component selected");

        assert_eq!(selected.name, "socket");
    }

    #[test]
    fn reliability_profile_increases_component_event_chance() {
        let variability = VariabilityProfile {
            descriptor: "Speed team".to_string(),
            issue_ids: "REL-JAM REL-FORCE".to_string(),
        };

        assert!(
            component_event_chance("loose prop", Some(&variability))
                > component_event_chance("", None)
        );
    }

    fn row(values: &[(&str, &str)]) -> HashMap<String, String> {
        values
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    fn component(values: &[(&str, &str)]) -> Component {
        Component::from_row(&row(values))
    }
}
