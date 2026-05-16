#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SimRunScoreInput {
    pub(crate) target_minutes: u32,
    pub(crate) run_minutes: u32,
    pub(crate) hints: u32,
    pub(crate) events: u32,
    pub(crate) over_slow: bool,
    pub(crate) admin_fail: bool,
    pub(crate) beat_count: u32,
}

pub(crate) fn score_simulated_run(input: SimRunScoreInput) -> u32 {
    let mut score: i32 = 100;

    if input.run_minutes > input.target_minutes {
        score -= ((input.run_minutes - input.target_minutes) * 2).min(24) as i32;
    }
    if input.over_slow {
        score -= 10;
    }
    if input.admin_fail {
        score -= 15;
    }

    score -= (input.events * 2).min(18) as i32;

    let expected_hint_budget = (input.beat_count / 2).max(1);
    if input.hints > expected_hint_budget {
        score -= ((input.hints - expected_hint_budget) * 2).min(16) as i32;
    }

    score.clamp(0, 100) as u32
}

#[cfg(test)]
mod tests {
    use super::{score_simulated_run, SimRunScoreInput};

    #[test]
    fn scores_clean_simulation_run_high() {
        assert_eq!(
            score_simulated_run(SimRunScoreInput {
                target_minutes: 45,
                run_minutes: 40,
                hints: 1,
                events: 0,
                over_slow: false,
                admin_fail: false,
                beat_count: 5,
            }),
            100
        );
    }

    #[test]
    fn penalizes_overrun_events_hints_and_failed_recovery() {
        let score = score_simulated_run(SimRunScoreInput {
            target_minutes: 45,
            run_minutes: 50,
            hints: 5,
            events: 3,
            over_slow: true,
            admin_fail: true,
            beat_count: 5,
        });

        assert!(score < 60);
    }
}
