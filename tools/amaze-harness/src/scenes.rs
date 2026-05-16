use crate::domain::BehaviorProbe;

pub(crate) fn behavior_for_team(
    behavior_probes: Option<&[BehaviorProbe]>,
    team_name: &str,
) -> String {
    let Some(probes) = behavior_probes else {
        return "not specified".to_string();
    };
    let team_lower = team_name.to_lowercase();
    probes
        .iter()
        .find(|probe| probe.team_persona.to_lowercase().contains(&team_lower))
        .map(|probe| probe.behavior.clone())
        .unwrap_or_else(|| "not specified".to_string())
}

#[cfg(test)]
mod tests {
    use super::behavior_for_team;
    use crate::domain::BehaviorProbe;

    #[test]
    fn finds_behavior_for_team() {
        let probes = vec![BehaviorProbe {
            behavior: "brute force".to_string(),
            team_persona: "Speed team".to_string(),
        }];
        assert_eq!(
            behavior_for_team(Some(&probes), "Speed team"),
            "brute force"
        );
    }
}
