use std::collections::HashMap;

use crate::cli::SimulateArgs;
use crate::domain::{BeatCard, Component, TransformationState, UnlockPath};
use crate::markdown::md_cell;
use crate::room::{read_optional_build_doc, read_optional_playtest_doc, read_scenes_doc};
use crate::rules::scoring::{score_simulated_run, SimRunScoreInput};
use crate::rules::simulation::{
    choose_chaos, choose_component_event, choose_team, choose_variability, component_event_chance,
    component_failure_mode, event_chance, first_issue_id, has_recovery_path, replacement_minutes,
    replacement_window, simulate_build_pressure, spare_qty,
};
use crate::rules::unlocks::{beat_id, is_tbd, unlock_beat_ids};
use crate::scenes::behavior_for_team;
use crate::sim::{percent, sample_between, Lcg};

pub(crate) fn simulate_room(args: SimulateArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let playtest_doc = read_optional_playtest_doc(&args.room);
    let build_doc = read_optional_build_doc(&args.room);
    let beat_cards = scenes_doc.require_beat_cards()?;
    if beat_cards.is_empty() {
        return Err("SCENES.md Beat cards table has no rows".to_string());
    }

    let mut rng = Lcg::new(args.seed);
    let mut completed = 0;
    let mut over_target = 0;
    let mut over_slow_runs = 0;
    let mut total_minutes = 0;
    let mut total_hints = 0;
    let mut total_events = 0;
    let mut total_admin_recoveries = 0;
    let mut failed_admin_recoveries = 0;
    let mut total_score = 0;
    let mut lowest_score = 100;
    let mut lowest_score_run = 0;
    let mut worst_run = 0;
    let mut worst_minutes = 0;
    let mut issue_counts: HashMap<String, u32> = HashMap::new();
    let mut component_counts: HashMap<String, u32> = HashMap::new();

    println!("# AMAZE Randomized Simulation");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!("| Runs | {} |", args.runs);
    println!("| Seed | {} |", args.seed);
    println!("| Target session | {} min |", args.target_minutes);
    println!();
    println!("## Run Samples");
    println!();
    println!("| Run | Team | Behavior | Minutes | Hints | Events | Score | Status |");
    println!("|---:|---|---|---:|---:|---|---:|---|");

    for run_index in 1..=args.runs {
        let team = choose_team(scenes_doc.team_probes.as_deref(), &mut rng);
        let behavior = behavior_for_team(scenes_doc.behavior_probes.as_deref(), &team);
        let run_variability = choose_variability(
            playtest_doc.variability_profiles.as_deref(),
            &team,
            &behavior,
            &mut rng,
        );
        let run_chaos = choose_chaos(playtest_doc.chaos_probes.as_deref(), &mut rng);
        let mut run_minutes = 0;
        let mut hints = 0;
        let mut over_slow = false;
        let mut events = Vec::new();
        let mut run_event_count = 0;
        let mut admin_fail = false;

        for beat in beat_cards {
            let target = beat.target_or_default();
            let slow = beat.slow_or_default();
            let event_chance = event_chance(
                &team,
                &behavior,
                &beat.behavior_probe,
                &beat.reliability_risk,
            );
            let mut actual = sample_between(&mut rng, target.saturating_sub(1).max(1), slow + 1);

            if rng.percent(event_chance) {
                actual += 1 + rng.next_u32(3);
                total_events += 1;
                run_event_count += 1;
                let issue = first_issue_id(&beat.reliability_risk)
                    .or_else(|| {
                        run_variability.and_then(|profile| first_issue_id(&profile.issue_ids))
                    })
                    .unwrap_or_else(|| "REL-STATE".to_string());
                *issue_counts.entry(issue.clone()).or_insert(0) += 1;
                events.push(issue);
            }

            let component_event_chance =
                component_event_chance(&beat.reliability_risk, run_variability);
            if let Some(component) = choose_component_event(
                build_doc.components.as_deref(),
                &beat.name,
                component_event_chance,
                &mut rng,
            ) {
                total_events += 1;
                run_event_count += 1;
                let mode = component_failure_mode(component, &beat.reliability_risk, &mut rng);
                let recovery_minutes = replacement_minutes(component);
                let has_spare = spare_qty(component) > 0;
                let has_bypass = has_recovery_path(component);
                let admin_in_time = has_bypass
                    || (has_spare && recovery_minutes <= replacement_window(target, slow));
                let event_label = if admin_in_time {
                    total_admin_recoveries += 1;
                    actual += recovery_minutes.max(1);
                    format!("ADMIN-RECOVERED {} {mode}", component.inventory_id)
                } else {
                    failed_admin_recoveries += 1;
                    admin_fail = true;
                    actual += recovery_minutes.max(slow.saturating_sub(target) + 2);
                    format!("ADMIN-LATE {} {mode}", component.inventory_id)
                };
                *component_counts
                    .entry(format!("{} ({})", component.name, component.inventory_id))
                    .or_insert(0) += 1;
                events.push(event_label);
            }

            if let Some(hint_at) = beat.hint_at_minutes {
                if actual > target && actual >= hint_at {
                    hints += 1;
                    if actual > slow {
                        actual = slow;
                    }
                }
            }
            if actual > slow {
                over_slow = true;
            }
            run_minutes += actual;
        }

        if let Some(chaos) = run_chaos {
            if rng.percent(35) {
                total_events += 1;
                run_event_count += 1;
                events.push(chaos.probe_id.clone());
                run_minutes += 1;
            }
        }

        completed += 1;
        total_minutes += run_minutes;
        total_hints += hints;
        if run_minutes > args.target_minutes {
            over_target += 1;
        }
        if admin_fail {
            over_slow = true;
        }
        if over_slow {
            over_slow_runs += 1;
        }
        if run_minutes > worst_minutes {
            worst_minutes = run_minutes;
            worst_run = run_index;
        }
        let run_score = score_simulated_run(SimRunScoreInput {
            target_minutes: args.target_minutes,
            run_minutes,
            hints,
            events: run_event_count,
            over_slow,
            admin_fail,
            beat_count: beat_cards.len() as u32,
        });
        total_score += run_score;
        if run_score < lowest_score {
            lowest_score = run_score;
            lowest_score_run = run_index;
        }

        if run_index <= 20 || run_index == args.runs {
            println!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                run_index,
                md_cell(&team),
                md_cell(&behavior),
                run_minutes,
                hints,
                md_cell(&events.join(", ")),
                run_score,
                if run_minutes > args.target_minutes {
                    "over target"
                } else {
                    "fits"
                }
            );
        }
    }

    println!();
    println!("## Simulation Summary");
    println!();
    println!("| Metric | Value |");
    println!("|---|---:|");
    println!("| Completed runs | {} |", completed);
    println!(
        "| Average minutes | {:.1} |",
        total_minutes as f64 / completed as f64
    );
    println!(
        "| Average hints | {:.1} |",
        total_hints as f64 / completed as f64
    );
    println!(
        "| Average run score | {:.1} |",
        total_score as f64 / completed as f64
    );
    println!("| Over target runs | {} |", over_target);
    println!(
        "| Over target rate | {:.0}% |",
        percent(over_target, completed)
    );
    println!("| Runs with over-slow beat | {} |", over_slow_runs);
    println!("| Reliability/chaos events | {} |", total_events);
    println!("| Admin recoveries in time | {} |", total_admin_recoveries);
    println!(
        "| Admin recoveries late/failed | {} |",
        failed_admin_recoveries
    );
    println!("| Worst run | {} ({} min) |", worst_run, worst_minutes);
    println!(
        "| Lowest score run | {} ({}) |",
        lowest_score_run, lowest_score
    );

    if let Some(components) = build_doc.components.as_ref() {
        print_build_time_simulation(components, args.runs, args.seed);
    }

    print_unlock_coherence(
        beat_cards,
        scenes_doc.transformation_states.as_deref(),
        scenes_doc.unlock_paths.as_deref(),
        args.target_minutes,
    );

    if !issue_counts.is_empty() {
        println!();
        println!("## Event Pressure");
        println!();
        println!("| Issue/probe | Count |");
        println!("|---|---:|");
        let mut issues: Vec<_> = issue_counts.into_iter().collect();
        issues.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        for (issue, count) in issues {
            println!("| {} | {} |", issue, count);
        }
    }

    if !component_counts.is_empty() {
        println!();
        println!("## Component Break/Loss Pressure");
        println!();
        println!("| Component | Events |");
        println!("|---|---:|");
        let mut components: Vec<_> = component_counts.into_iter().collect();
        components.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        for (component, count) in components {
            println!("| {} | {} |", md_cell(&component), count);
        }
    }

    println!();
    println!("## Design Decisions");
    println!();
    if over_target * 5 > completed {
        println!("- More than 20% of randomized runs exceeded target; add acceleration paths or cut variance.");
    }
    let high_hint_threshold = completed * (beat_cards.len() as u32).max(1) * 3 / 5;
    if total_hints > high_hint_threshold {
        println!(
            "- Average hint load is high; strengthen onboarding, affordances, or early feedback."
        );
    }
    if total_events > completed {
        println!("- Reliability/chaos pressure is high; run bench tests before adding scope.");
    }
    if failed_admin_recoveries > 0 {
        println!(
            "- Some broken/lost components were not recoverable in time; increase spares, lower replacement minutes, tether parts, or add an explicit bypass."
        );
    }
    if over_target == 0 && total_events <= completed / 2 {
        println!("- Randomized timing looks healthy enough to test with real team archetypes.");
    }

    Ok(())
}

fn print_build_time_simulation(components: &[Component], runs: u32, seed: u64) {
    let Some(pressure) = simulate_build_pressure(components, runs, seed) else {
        return;
    };

    println!();
    println!("## Build Time Pressure");
    println!();
    println!("| Metric | Hours |");
    println!("|---|---:|");
    println!(
        "| Simulated average build | {:.1} |",
        pressure.average_hours
    );
    println!("| Fast build case | {} |", pressure.min_hours);
    println!("| Slow build case | {} |", pressure.max_hours);

    println!();
    println!("| Likely bottleneck item | Runs as bottleneck |");
    println!("|---|---:|");
    for (name, count) in pressure.bottlenecks.into_iter().take(5) {
        println!("| {} | {} |", md_cell(&name), count);
    }
}

fn print_unlock_coherence(
    beat_cards: &[BeatCard],
    transformations: Option<&[TransformationState]>,
    unlock_paths: Option<&[UnlockPath]>,
    target_minutes: u32,
) {
    let Some(paths) = unlock_paths.filter(|paths| !paths.is_empty()) else {
        return;
    };

    let transformation_count = transformations.map_or(0, <[TransformationState]>::len);

    println!();
    println!("## Unlock Coherence");
    println!();
    println!("| Metric | Value |");
    println!("|---|---:|");
    println!("| Transformation states | {} |", transformation_count);
    println!("| Unlock paths | {} |", paths.len());
    println!();
    println!("| Path | Beats | Unlocks | Fast unlock min | Slow unlock min | Fast coherence | Slow coherence | Operator acceleration | Status |");
    println!("|---|---|---|---:|---:|---|---|---|---|");

    for path in paths {
        let beat_ids = unlock_beat_ids(&path.beats);
        let fast_minutes = sum_beat_minutes(beat_cards, &beat_ids, BeatPace::Fast);
        let slow_minutes = sum_beat_minutes(beat_cards, &beat_ids, BeatPace::Slow);
        let status = unlock_status(path, slow_minutes, target_minutes);

        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            md_cell(&path.path),
            md_cell(&path.beats),
            md_cell(&path.unlocks),
            fast_minutes,
            slow_minutes,
            md_cell(&path.fast_coherence),
            md_cell(&path.slow_coherence),
            md_cell(&path.operator_acceleration),
            status
        );
    }
}

enum BeatPace {
    Fast,
    Slow,
}

fn sum_beat_minutes(beat_cards: &[BeatCard], beat_ids: &[String], pace: BeatPace) -> u32 {
    beat_ids
        .iter()
        .filter_map(|id| {
            beat_cards
                .iter()
                .find(|beat| beat_id(&beat.name).is_some_and(|beat_id| beat_id == id.as_str()))
        })
        .map(|beat| match pace {
            BeatPace::Fast => beat.target_or_default(),
            BeatPace::Slow => beat.slow_or_default(),
        })
        .sum()
}

fn unlock_status(path: &UnlockPath, slow_minutes: u32, target_minutes: u32) -> &'static str {
    if is_tbd(&path.fast_coherence) || is_tbd(&path.slow_coherence) {
        "review coherence text"
    } else if is_tbd(&path.operator_acceleration) {
        "missing accelerator"
    } else if slow_minutes > target_minutes {
        "slow path exceeds target"
    } else {
        "declared"
    }
}
