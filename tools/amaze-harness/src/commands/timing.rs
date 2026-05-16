use crate::cli::{AnalyzeArgs, MatrixArgs, OptimizeArgs, RecordArgs, RunArgs};
use crate::markdown::{append_rows_to_section, md_cell};
use crate::room::{read_room_file, read_scenes_doc, write_room_file, PLAYTEST_FILE};
use crate::rules::timing::{
    acceleration_candidates, actual_for_beat, delta_status, fit_status, hint_used_at, slack_status,
    timing_status, BeatTiming,
};
use crate::scenes::behavior_for_team;

pub(crate) fn run_room(args: RunArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let beat_cards = scenes_doc.require_beat_cards()?;

    println!("# AMAZE Beat Run Sheet");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!("| Team archetype | {} |", args.team);
    println!(
        "| Behavior focus | {} |",
        args.behavior.as_deref().unwrap_or("not specified")
    );
    println!(
        "| Session clock | {} |",
        args.clock_minutes
            .map(|minutes| format!("{minutes} minutes"))
            .unwrap_or_else(|| "room default".to_string())
    );
    println!();

    if let Some(scenes) = scenes_doc.scene_summaries.as_ref() {
        if !scenes.is_empty() {
            println!("## Scene Order");
            println!();
            println!("| # | Scene | Purpose | Clock | Team/behavior probes |");
            println!("|---:|---|---|---|---|");
            for (index, scene) in scenes.iter().enumerate() {
                println!(
                    "| {} | {} | {} | {} | {} |",
                    index + 1,
                    scene.scene,
                    scene.purpose,
                    scene.clock,
                    scene.team_behavior_probes
                );
            }
            println!();
        }
    }

    println!("## Beat Walk");
    println!();
    if beat_cards.is_empty() {
        println!("No beats found. Fill in `SCENES.md` -> `## Beat cards` first.");
        return Ok(());
    }

    for (index, beat) in beat_cards.iter().enumerate() {
        println!("### Beat {}: {}", index + 1, beat.name);
        println!();
        println!("| Prompt | Value |");
        println!("|---|---|");
        println!("| Scene | {} |", beat.scene);
        println!("| Player action | {} |", beat.player_action);
        println!("| Aha | {} |", beat.aha);
        println!("| Check | {} |", beat.check);
        println!("| Behavior probe | {} |", beat.behavior_probe);
        println!("| Target min | {} |", display_minutes(beat.target_minutes));
        println!(
            "| Hint at min | {} |",
            display_minutes(beat.hint_at_minutes)
        );
        println!("| Slow max min | {} |", display_minutes(beat.slow_minutes));
        println!("| Mechanism | {} |", beat.mechanism);
        println!("| Reliability risk | {} |", beat.reliability_risk);
        println!("| DC | {} |", beat.dc);
        println!("| Success | {} |", beat.success);
        println!("| Partial | {} |", beat.partial);
        println!("| Stall/hint trigger | {} |", beat.stall_hint_trigger);
        println!("| Reset effect | {} |", beat.reset_effect);
        println!();
        println!("Observer quick-log: signal, team outcome, behavior severity, operator response, revision.");
        println!();
    }

    println!("## End-of-Run Decisions");
    println!();
    println!("| Question | Decision |");
    println!("|---|---|");
    println!("| Did this team archetype expose a revise/block issue? | TBD |");
    println!("| Did any beat need a hint earlier than designed? | TBD |");
    println!("| Did any mechanism create reliability, safety, or reset debt? | TBD |");
    println!("| What is the top revision before adding new scope? | TBD |");

    Ok(())
}

pub(crate) fn optimize_room(args: OptimizeArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let beat_cards = scenes_doc.require_beat_cards()?;

    if beat_cards.is_empty() {
        println!("# AMAZE Session Optimizer");
        println!();
        println!("No beats found. Fill in `SCENES.md` -> `## Beat cards` with `Target min`, `Hint at min`, and `Slow max min` to optimize a session.");
        return Ok(());
    }

    let mut total_target = 0;
    let mut total_slow = 0;
    let mut missing_timing = Vec::new();
    let mut late_hint = Vec::new();
    let mut timings = Vec::new();

    println!("# AMAZE Session Optimizer");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!("| Target session | {} min |", args.target_minutes);
    println!("| Beat count | {} |", beat_cards.len());
    println!();
    println!("## Beat Timing");
    println!();
    println!("| # | Beat | Scene | Target min | Hint at min | Slow max min | Pressure | Recommendation |");
    println!("|---:|---|---|---:|---:|---:|---|---|");

    for (index, beat) in beat_cards.iter().enumerate() {
        if beat.target_minutes.is_none() {
            missing_timing.push(beat.name.clone());
        }

        let target_value = beat.target_minutes.unwrap_or(0);
        let slow_value = beat.slow_minutes.unwrap_or(target_value);
        total_target += target_value;
        total_slow += slow_value;
        if beat.target_minutes.is_some() {
            timings.push(BeatTiming {
                name: beat.name.clone(),
                target: target_value,
                slow: slow_value,
            });
        }

        let pressure = if slow_value > target_value.saturating_mul(2) && target_value > 0 {
            "high variance"
        } else if target_value >= 6 {
            "long beat"
        } else if target_value == 0 {
            "unknown"
        } else {
            "normal"
        };

        let recommendation = match (beat.target_minutes, beat.hint_at_minutes, beat.slow_minutes) {
            (None, _, _) => "add timing budget",
            (Some(target), Some(hint), _) if hint >= target => {
                late_hint.push(beat.name.clone());
                "move hint before target time"
            }
            (Some(_), None, _) => "add hint trigger minute",
            (Some(target), _, Some(max)) if max >= target + 3 => "add acceleration path",
            _ => "ok",
        };

        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            index + 1,
            beat.name,
            beat.scene,
            display_minutes(beat.target_minutes),
            display_minutes(beat.hint_at_minutes),
            display_minutes(beat.slow_minutes),
            pressure,
            recommendation
        );
    }

    let slack = args.target_minutes as i32 - total_target as i32;
    let slow_slack = args.target_minutes as i32 - total_slow as i32;

    println!();
    println!("## Session Fit");
    println!();
    println!("| Metric | Minutes | Status |");
    println!("|---|---:|---|");
    println!(
        "| Planned target total | {} | {} |",
        total_target,
        fit_status(slack)
    );
    println!(
        "| Slow-case total | {} | {} |",
        total_slow,
        fit_status(slow_slack)
    );
    println!("| Target session | {} | baseline |", args.target_minutes);
    println!("| Planned slack | {} | {} |", slack, slack_status(slack));
    println!(
        "| Slow-case slack | {} | {} |",
        slow_slack,
        slack_status(slow_slack)
    );

    println!();
    println!("## Optimization Notes");
    println!();
    if missing_timing.is_empty() && late_hint.is_empty() && slack >= 0 {
        println!(
            "- Timing model is usable for a first {}-minute simulation.",
            args.target_minutes
        );
    }
    if !missing_timing.is_empty() {
        println!("- Add timing budgets for: {}.", missing_timing.join(", "));
    }
    if !late_hint.is_empty() {
        println!(
            "- Move hint triggers earlier than target time for: {}.",
            late_hint.join(", ")
        );
    }
    if slack < 0 {
        println!(
            "- Cut {} planned minutes or split/accelerate beats before playtest.",
            slack.abs()
        );
    } else if slack < 3 {
        println!("- Add at least 3 minutes of buffer for intro, transitions, and human variance.");
    }
    if slow_slack < 0 {
        println!(
            "- Slow-case run exceeds target by {} minutes; add hard acceleration paths.",
            slow_slack.abs()
        );
        let candidates = acceleration_candidates(&timings, slow_slack.abs() as u32);
        if !candidates.is_empty() {
            println!("- Best acceleration candidates: {}.", candidates.join(", "));
        }
    }

    Ok(())
}

pub(crate) fn analyze_run(args: AnalyzeArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let beat_cards = scenes_doc.require_beat_cards()?;

    if beat_cards.is_empty() {
        println!("# AMAZE Run Timing Analysis");
        println!();
        println!("No beats found. Fill in `SCENES.md` -> `## Beat cards` before analyzing a run.");
        return Ok(());
    }

    let mut actual_total = 0;
    let mut target_total = 0;
    let mut missing_actuals = Vec::new();
    let mut over_slow = Vec::new();
    let mut hint_pressure = Vec::new();

    println!("# AMAZE Run Timing Analysis");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!(
        "| Team archetype | {} |",
        args.team.as_deref().unwrap_or("not specified")
    );
    println!("| Target session | {} min |", args.target_minutes);
    println!();
    println!("## Beat Timing vs Actual");
    println!();
    println!("| Beat | Target min | Actual min | Delta | Hint at min | Slow max min | Status |");
    println!("|---|---:|---:|---:|---:|---:|---|");

    for beat in beat_cards {
        let actual = actual_for_beat(&args.actual_times, &beat.name);

        if let Some(target) = beat.target_minutes {
            target_total += target;
        }

        let status = match (
            actual,
            beat.target_minutes,
            beat.hint_at_minutes,
            beat.slow_minutes,
        ) {
            (None, _, _, _) => {
                missing_actuals.push(beat.name.clone());
                "missing actual"
            }
            (Some(actual), _, _, Some(slow)) if actual > slow => {
                over_slow.push(beat.name.clone());
                actual_total += actual;
                "over slow max"
            }
            (Some(actual), Some(target), Some(hint), _) if actual > target && actual >= hint => {
                hint_pressure.push(beat.name.clone());
                actual_total += actual;
                "hint pressure"
            }
            (Some(actual), Some(target), _, _) if actual > target => {
                actual_total += actual;
                "over target"
            }
            (Some(actual), _, _, _) => {
                actual_total += actual;
                "ok"
            }
        };

        let delta = match (actual, beat.target_minutes) {
            (Some(actual), Some(target)) => actual as i32 - target as i32,
            _ => 0,
        };

        println!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            beat.name,
            display_minutes(beat.target_minutes),
            display_minutes(actual),
            delta,
            display_minutes(beat.hint_at_minutes),
            display_minutes(beat.slow_minutes),
            status
        );
    }

    let session_delta = actual_total as i32 - args.target_minutes as i32;
    let plan_delta = actual_total as i32 - target_total as i32;

    println!();
    println!("## Session Timing");
    println!();
    println!("| Metric | Minutes | Status |");
    println!("|---|---:|---|");
    println!("| Planned beat total | {} | baseline |", target_total);
    println!(
        "| Actual beat total | {} | {} |",
        actual_total,
        fit_status(-session_delta)
    );
    println!(
        "| Actual vs planned beats | {} | {} |",
        plan_delta,
        delta_status(plan_delta)
    );
    println!(
        "| Actual vs session target | {} | {} |",
        session_delta,
        delta_status(session_delta)
    );

    println!();
    println!("## Timing Decisions");
    println!();
    if missing_actuals.is_empty()
        && over_slow.is_empty()
        && hint_pressure.is_empty()
        && session_delta <= 0
    {
        println!("- Run timing is healthy enough to keep testing other team archetypes.");
    }
    if !missing_actuals.is_empty() {
        println!(
            "- Missing actual times for: {}.",
            missing_actuals.join(", ")
        );
    }
    if !over_slow.is_empty() {
        println!(
            "- Beats over slow max need cut, acceleration, or redesign: {}.",
            over_slow.join(", ")
        );
    }
    if !hint_pressure.is_empty() {
        println!(
            "- Beats reached hint pressure; verify hints fired cleanly: {}.",
            hint_pressure.join(", ")
        );
    }
    if session_delta > 0 {
        println!(
            "- Run exceeded target by {} minutes; recover time before adding scope.",
            session_delta
        );
    }

    Ok(())
}

pub(crate) fn record_run(args: RecordArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let beat_cards = scenes_doc.require_beat_cards()?;
    if beat_cards.is_empty() {
        return Err("no beat rows found in SCENES.md".to_string());
    }

    let playtest = read_room_file(&args.room, PLAYTEST_FILE)?;

    let mut rows = Vec::new();
    let mut actual_total = 0;
    let mut missing_actuals = Vec::new();

    for beat in beat_cards {
        let actual = actual_for_beat(&args.actual_times, &beat.name);
        if let Some(actual) = actual {
            actual_total += actual;
        } else {
            missing_actuals.push(beat.name.clone());
        }

        rows.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            md_cell(&args.run_id),
            md_cell(&beat.name),
            display_minutes(beat.target_minutes),
            display_minutes(actual),
            display_minutes(beat.hint_at_minutes),
            display_minutes(hint_used_at(actual, beat.hint_at_minutes)),
            display_minutes(beat.slow_minutes),
            timing_status(
                actual,
                beat.target_minutes,
                beat.hint_at_minutes,
                beat.slow_minutes
            )
        ));
    }

    let finish = args.finish.unwrap_or(actual_total);
    let summary_row = format!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        md_cell(&args.run_id),
        md_cell(args.version.as_deref().unwrap_or("TBD")),
        md_cell(args.date.as_deref().unwrap_or("TBD")),
        md_cell(args.run_type.as_deref().unwrap_or("simulation")),
        md_cell(args.team.as_deref().unwrap_or("TBD")),
        md_cell(args.personas.as_deref().unwrap_or("TBD")),
        display_optional_u32(args.players),
        finish,
        display_optional_u32(args.hints),
        md_cell(args.stuck.as_deref().unwrap_or("TBD")),
        md_cell(args.failures.as_deref().unwrap_or("TBD")),
        md_cell(args.safety.as_deref().unwrap_or("TBD")),
        display_optional_u32(args.reset),
        md_cell(args.memorable.as_deref().unwrap_or("TBD"))
    );

    let updated =
        append_rows_to_section(&playtest, "## Simulation and playtest runs", &[summary_row])?;
    let updated = append_rows_to_section(&updated, "## Beat timing evidence", &rows)?;
    write_room_file(&args.room, PLAYTEST_FILE, &updated)?;

    println!("# AMAZE Record Timing");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!("| Run ID | {} |", args.run_id);
    println!(
        "| Team archetype | {} |",
        args.team.as_deref().unwrap_or("not specified")
    );
    println!("| Finish time | {} min |", finish);
    println!("| Actual beat total | {} min |", actual_total);
    println!("| Target session | {} min |", args.target_minutes);
    println!("| PLAYTEST.md updated | yes |");
    if !missing_actuals.is_empty() {
        println!("| Missing actuals | {} |", missing_actuals.join(", "));
    }

    Ok(())
}

pub(crate) fn matrix_room(args: MatrixArgs) -> Result<(), String> {
    let scenes_doc = read_scenes_doc(&args.room)?;
    let team_probes = scenes_doc.require_team_probes()?;

    if team_probes.is_empty() {
        println!("# AMAZE Team Run Matrix");
        println!();
        println!("No team archetype probes found. Fill in `SCENES.md` -> `## Team archetype probes` first.");
        return Ok(());
    }

    println!("# AMAZE Team Run Matrix");
    println!();
    println!("| Field | Value |");
    println!("|---|---|");
    println!("| Room | {} |", args.room.display());
    println!("| Target session | {} min |", args.target_minutes);
    println!("| Team passes | {} |", team_probes.len());
    println!();
    println!("## Required Team Passes");
    println!();
    println!("| # | Team archetype | Probe | Observable signal | Suggested behavior focus |");
    println!("|---:|---|---|---|---|");

    for (index, team) in team_probes.iter().enumerate() {
        let behavior_focus =
            behavior_for_team(scenes_doc.behavior_probes.as_deref(), &team.archetype);
        println!(
            "| {} | {} | {} | {} | {} |",
            index + 1,
            team.archetype,
            team.scene_beat_probe,
            team.observable_signal,
            behavior_focus
        );
    }

    println!();
    println!("## Run Commands");
    println!();
    for team in team_probes {
        let behavior_focus =
            behavior_for_team(scenes_doc.behavior_probes.as_deref(), &team.archetype);
        println!(
            "- `cargo run --manifest-path tools\\amaze-harness\\Cargo.toml -- run --room {} --team \"{}\" --behavior \"{}\" --clock {}`",
            args.room.display(),
            team.archetype,
            behavior_focus,
            args.target_minutes
        );
    }
    println!();
    println!("After each pass, analyze measured times with:");
    println!();
    println!(
        "`cargo run --manifest-path tools\\amaze-harness\\Cargo.toml -- analyze --room {} --team \"<team>\" --actual P1=?,P2=?,P3=?,P4=?,P5=? --target {}`",
        args.room.display(),
        args.target_minutes
    );

    Ok(())
}

fn display_optional_u32(value: Option<u32>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "TBD".to_string())
}

fn display_minutes(value: Option<u32>) -> String {
    value
        .map(|minutes| minutes.to_string())
        .unwrap_or_else(|| "TBD".to_string())
}
