use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct BeatTiming {
    pub(crate) name: String,
    pub(crate) target: u32,
    pub(crate) slow: u32,
}

pub(crate) fn acceleration_candidates(timings: &[BeatTiming], needed_minutes: u32) -> Vec<String> {
    let mut ranked: Vec<&BeatTiming> = timings
        .iter()
        .filter(|timing| timing.slow > timing.target)
        .collect();
    ranked.sort_by(|left, right| {
        let left_variance = left.slow - left.target;
        let right_variance = right.slow - right.target;
        right_variance.cmp(&left_variance)
    });

    let mut recovered = 0;
    let mut candidates = Vec::new();
    for timing in ranked {
        let variance = timing.slow - timing.target;
        recovered += variance;
        candidates.push(format!("{} (-{} min)", timing.name, variance));
        if recovered >= needed_minutes {
            break;
        }
    }
    candidates
}

pub(crate) fn actual_for_beat(actual_times: &HashMap<String, u32>, beat_name: &str) -> Option<u32> {
    actual_times.get(beat_name).copied().or_else(|| {
        beat_name
            .split_whitespace()
            .next()
            .and_then(|id| actual_times.get(id).copied())
    })
}

pub(crate) fn timing_status(
    actual: Option<u32>,
    target: Option<u32>,
    hint_at: Option<u32>,
    slow: Option<u32>,
) -> &'static str {
    match (actual, target, hint_at, slow) {
        (None, _, _, _) => "missing actual",
        (Some(actual), _, _, Some(slow)) if actual > slow => "over slow max",
        (Some(actual), Some(target), Some(hint), _) if actual > target && actual >= hint => {
            "hint pressure"
        }
        (Some(actual), Some(target), _, _) if actual > target => "over target",
        (Some(_), _, _, _) => "ok",
    }
}

pub(crate) fn hint_used_at(actual: Option<u32>, hint_at: Option<u32>) -> Option<u32> {
    match (actual, hint_at) {
        (Some(actual), Some(hint)) if actual >= hint => Some(hint),
        _ => None,
    }
}

pub(crate) fn fit_status(slack: i32) -> &'static str {
    if slack < 0 {
        "over target"
    } else if slack < 3 {
        "tight"
    } else {
        "fits"
    }
}

pub(crate) fn slack_status(slack: i32) -> &'static str {
    if slack < 0 {
        "cut or accelerate"
    } else if slack < 3 {
        "too little buffer"
    } else if slack > 8 {
        "room for payoff or optional path"
    } else {
        "healthy buffer"
    }
}

pub(crate) fn delta_status(delta: i32) -> &'static str {
    if delta > 0 {
        "over"
    } else if delta < 0 {
        "under"
    } else {
        "on target"
    }
}

#[cfg(test)]
mod tests {
    use super::actual_for_beat;
    use crate::cli::parse_actual_times;

    #[test]
    fn matches_actual_by_beat_id_prefix() {
        let times = parse_actual_times("P1=4").expect("valid timings");
        assert_eq!(actual_for_beat(&times, "P1 route fragments"), Some(4));
    }
}
