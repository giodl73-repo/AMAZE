use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const REQUIRED_ROOM_FILES: &[&str] = &[
    "BRIEF.md",
    "FLOORPLAN.md",
    "PUZZLE-GRAPH.md",
    "SCENES.md",
    "BUILD.md",
    "SAFETY.md",
    "PLAYTEST.md",
    "OPS.md",
];
const DEFAULT_TARGET_MINUTES: u32 = 45;

#[derive(Debug)]
struct Cli {
    command: Command,
}

#[derive(Debug)]
enum Command {
    Run(RunArgs),
    Optimize(OptimizeArgs),
    Analyze(AnalyzeArgs),
    Record(RecordArgs),
    Matrix(MatrixArgs),
    Inventory(InventoryArgs),
    Check { room: PathBuf },
    Help,
}

#[derive(Debug)]
struct RunArgs {
    room: PathBuf,
    team: String,
    behavior: Option<String>,
    clock_minutes: Option<u32>,
}

#[derive(Debug)]
struct OptimizeArgs {
    room: PathBuf,
    target_minutes: u32,
}

#[derive(Debug)]
struct AnalyzeArgs {
    room: PathBuf,
    team: Option<String>,
    target_minutes: u32,
    actual_times: HashMap<String, u32>,
}

#[derive(Debug)]
struct RecordArgs {
    room: PathBuf,
    run_id: String,
    version: Option<String>,
    date: Option<String>,
    run_type: Option<String>,
    team: Option<String>,
    personas: Option<String>,
    players: Option<u32>,
    finish: Option<u32>,
    hints: Option<u32>,
    stuck: Option<String>,
    failures: Option<String>,
    safety: Option<String>,
    reset: Option<u32>,
    memorable: Option<String>,
    target_minutes: u32,
    actual_times: HashMap<String, u32>,
}

#[derive(Debug)]
struct MatrixArgs {
    room: PathBuf,
    target_minutes: u32,
}

#[derive(Debug)]
struct InventoryArgs {
    catalog: PathBuf,
    query: Option<String>,
}

#[derive(Debug)]
struct BeatTiming {
    name: String,
    target: u32,
    slow: u32,
}

#[derive(Debug)]
struct Table {
    rows: Vec<HashMap<String, String>>,
}

fn main() {
    let cli = parse_cli(env::args().skip(1).collect()).unwrap_or_else(|err| {
        eprintln!("error: {err}\n");
        print_help();
        process::exit(2);
    });

    let result = match cli.command {
        Command::Run(args) => run_room(args),
        Command::Optimize(args) => optimize_room(args),
        Command::Analyze(args) => analyze_run(args),
        Command::Record(args) => record_run(args),
        Command::Matrix(args) => matrix_room(args),
        Command::Inventory(args) => inventory_catalog(args),
        Command::Check { room } => check_room(&room),
        Command::Help => {
            print_help();
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn parse_cli(args: Vec<String>) -> Result<Cli, String> {
    if args.is_empty() {
        return Ok(Cli {
            command: Command::Help,
        });
    }

    match args[0].as_str() {
        "run" => parse_run(&args[1..]).map(|command| Cli { command }),
        "optimize" => parse_optimize(&args[1..]).map(|command| Cli { command }),
        "analyze" => parse_analyze(&args[1..]).map(|command| Cli { command }),
        "record" => parse_record(&args[1..]).map(|command| Cli { command }),
        "matrix" => parse_matrix(&args[1..]).map(|command| Cli { command }),
        "inventory" => parse_inventory(&args[1..]).map(|command| Cli { command }),
        "check" => parse_check(&args[1..]).map(|command| Cli { command }),
        "help" | "-h" | "--help" => Ok(Cli {
            command: Command::Help,
        }),
        other => Err(format!("unknown command '{other}'")),
    }
}

fn parse_matrix(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut target_minutes = Some(DEFAULT_TARGET_MINUTES);
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--target" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--target requires minutes".to_string())?;
                target_minutes = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("invalid --target value '{raw}'"))?,
                );
            }
            other => return Err(format!("unknown matrix argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Matrix(MatrixArgs {
        room: room.ok_or_else(|| "matrix requires --room <path>".to_string())?,
        target_minutes: target_minutes.unwrap_or(DEFAULT_TARGET_MINUTES),
    }))
}

fn parse_analyze(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut team = None;
    let mut target_minutes = Some(DEFAULT_TARGET_MINUTES);
    let mut actual_times = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--team" => {
                i += 1;
                team = args.get(i).cloned();
            }
            "--target" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--target requires minutes".to_string())?;
                target_minutes = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("invalid --target value '{raw}'"))?,
                );
            }
            "--actual" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--actual requires comma-separated beat timings".to_string())?;
                actual_times = Some(parse_actual_times(raw)?);
            }
            other => return Err(format!("unknown analyze argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Analyze(AnalyzeArgs {
        room: room.ok_or_else(|| "analyze requires --room <path>".to_string())?,
        team,
        target_minutes: target_minutes.unwrap_or(DEFAULT_TARGET_MINUTES),
        actual_times: actual_times
            .ok_or_else(|| "analyze requires --actual P1=4,P2=5,...".to_string())?,
    }))
}

fn parse_record(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut run_id = None;
    let mut version = None;
    let mut date = None;
    let mut run_type = None;
    let mut team = None;
    let mut personas = None;
    let mut players = None;
    let mut finish = None;
    let mut hints = None;
    let mut stuck = None;
    let mut failures = None;
    let mut safety = None;
    let mut reset = None;
    let mut memorable = None;
    let mut target_minutes = Some(DEFAULT_TARGET_MINUTES);
    let mut actual_times = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--run" => {
                i += 1;
                run_id = args.get(i).cloned();
            }
            "--version" => {
                i += 1;
                version = args.get(i).cloned();
            }
            "--date" => {
                i += 1;
                date = args.get(i).cloned();
            }
            "--type" => {
                i += 1;
                run_type = args.get(i).cloned();
            }
            "--team" => {
                i += 1;
                team = args.get(i).cloned();
            }
            "--personas" => {
                i += 1;
                personas = args.get(i).cloned();
            }
            "--players" => {
                i += 1;
                players = Some(parse_u32_arg(args.get(i), "--players")?);
            }
            "--finish" => {
                i += 1;
                finish = Some(parse_u32_arg(args.get(i), "--finish")?);
            }
            "--hints" => {
                i += 1;
                hints = Some(parse_u32_arg(args.get(i), "--hints")?);
            }
            "--stuck" => {
                i += 1;
                stuck = args.get(i).cloned();
            }
            "--failures" => {
                i += 1;
                failures = args.get(i).cloned();
            }
            "--safety" => {
                i += 1;
                safety = args.get(i).cloned();
            }
            "--reset" => {
                i += 1;
                reset = Some(parse_u32_arg(args.get(i), "--reset")?);
            }
            "--memorable" => {
                i += 1;
                memorable = args.get(i).cloned();
            }
            "--target" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--target requires minutes".to_string())?;
                target_minutes = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("invalid --target value '{raw}'"))?,
                );
            }
            "--actual" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--actual requires comma-separated beat timings".to_string())?;
                actual_times = Some(parse_actual_times(raw)?);
            }
            other => return Err(format!("unknown record argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Record(RecordArgs {
        room: room.ok_or_else(|| "record requires --room <path>".to_string())?,
        run_id: run_id.ok_or_else(|| "record requires --run <id>".to_string())?,
        version,
        date,
        run_type,
        team,
        personas,
        players,
        finish,
        hints,
        stuck,
        failures,
        safety,
        reset,
        memorable,
        target_minutes: target_minutes.unwrap_or(DEFAULT_TARGET_MINUTES),
        actual_times: actual_times
            .ok_or_else(|| "record requires --actual P1=4,P2=5,...".to_string())?,
    }))
}

fn parse_optimize(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut target_minutes = Some(DEFAULT_TARGET_MINUTES);
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--target" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--target requires minutes".to_string())?;
                target_minutes = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("invalid --target value '{raw}'"))?,
                );
            }
            other => return Err(format!("unknown optimize argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Optimize(OptimizeArgs {
        room: room.ok_or_else(|| "optimize requires --room <path>".to_string())?,
        target_minutes: target_minutes.unwrap_or(DEFAULT_TARGET_MINUTES),
    }))
}

fn parse_inventory(args: &[String]) -> Result<Command, String> {
    let mut catalog = PathBuf::from("components\\INVENTORY.md");
    let mut query = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--catalog" => {
                i += 1;
                catalog = args
                    .get(i)
                    .map(PathBuf::from)
                    .ok_or_else(|| "--catalog requires a path".to_string())?;
            }
            "--query" | "-q" => {
                i += 1;
                query = args.get(i).cloned();
            }
            other => return Err(format!("unknown inventory argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Inventory(InventoryArgs { catalog, query }))
}

fn parse_run(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut team = None;
    let mut behavior = None;
    let mut clock_minutes = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--team" => {
                i += 1;
                team = args.get(i).cloned();
            }
            "--behavior" => {
                i += 1;
                behavior = args.get(i).cloned();
            }
            "--clock" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--clock requires minutes".to_string())?;
                clock_minutes = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("invalid --clock value '{raw}'"))?,
                );
            }
            other => return Err(format!("unknown run argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Run(RunArgs {
        room: room.ok_or_else(|| "run requires --room <path>".to_string())?,
        team: team.ok_or_else(|| "run requires --team <team archetype>".to_string())?,
        behavior,
        clock_minutes,
    }))
}

fn parse_check(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            other => return Err(format!("unknown check argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Check {
        room: room.ok_or_else(|| "check requires --room <path>".to_string())?,
    })
}

fn print_help() {
    println!(
        "AMAZE harness\n\n\
         Commands:\n\
           amaze check --room <room-path>\n\
           amaze run --room <room-path> --team <team archetype> [--behavior <behavior>] [--clock <minutes>]\n\
           amaze optimize --room <room-path> [--target <minutes>]\n\
           amaze analyze --room <room-path> --actual P1=4,P2=5 [--team <team>] [--target <minutes>]\n\n\
            amaze record --room <room-path> --run <id> --actual P1=4,P2=5 [--team <team>] [--finish <minutes>] [--hints <count>]\n\
            amaze matrix --room <room-path> [--target <minutes>]\n\n\
            amaze inventory [--query <text>] [--catalog <path>]\n\n\
          Example:\n\
            cargo run --manifest-path tools\\amaze-harness\\Cargo.toml -- inventory --query latch"
     );
}

fn check_room(room: &Path) -> Result<(), String> {
    let mut failures = Vec::new();

    for file in REQUIRED_ROOM_FILES {
        let path = room.join(file);
        if !path.exists() {
            failures.push(format!("missing required room file: {}", path.display()));
        }
    }

    let scenes_path = room.join("SCENES.md");
    if scenes_path.exists() {
        let scenes = fs::read_to_string(&scenes_path)
            .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
        for heading in [
            "## Party assumptions",
            "## Team archetype probes",
            "## Behavior probes",
            "## Scene list",
            "## Beat cards",
            "## Operator interventions",
        ] {
            if !scenes.contains(heading) {
                failures.push(format!("SCENES.md missing heading: {heading}"));
            }
        }
    }

    let playtest_path = room.join("PLAYTEST.md");
    if playtest_path.exists() {
        let playtest = fs::read_to_string(&playtest_path)
            .map_err(|err| format!("failed to read {}: {err}", playtest_path.display()))?;
        for heading in [
            "## Simulation and playtest runs",
            "## Team archetype evidence",
            "## Behavior evidence",
            "## Stuck-state log",
            "## Scores",
        ] {
            if !playtest.contains(heading) {
                failures.push(format!("PLAYTEST.md missing heading: {heading}"));
            }
        }
    }

    if failures.is_empty() {
        println!("room check passed: {}", room.display());
        Ok(())
    } else {
        for failure in failures {
            eprintln!("- {failure}");
        }
        Err("room check failed".to_string())
    }
}

fn run_room(args: RunArgs) -> Result<(), String> {
    let scenes_path = args.room.join("SCENES.md");
    let scenes = fs::read_to_string(&scenes_path)
        .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
    let scene_table = extract_table(&scenes, "## Scene list");
    let beat_table = extract_table(&scenes, "## Beat cards")
        .ok_or_else(|| "SCENES.md does not contain a Beat cards table".to_string())?;

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

    if let Some(table) = scene_table {
        if !table.rows.is_empty() {
            println!("## Scene Order");
            println!();
            println!("| # | Scene | Purpose | Clock | Team/behavior probes |");
            println!("|---:|---|---|---|---|");
            for (index, row) in table.rows.iter().enumerate() {
                println!(
                    "| {} | {} | {} | {} | {} |",
                    index + 1,
                    value(row, "Scene"),
                    value(row, "Purpose"),
                    value(row, "Clock"),
                    value(row, "Team/behavior probes")
                );
            }
            println!();
        }
    }

    println!("## Beat Walk");
    println!();
    if beat_table.rows.is_empty() {
        println!("No beats found. Fill in `SCENES.md` -> `## Beat cards` first.");
        return Ok(());
    }

    for (index, beat) in beat_table.rows.iter().enumerate() {
        println!("### Beat {}: {}", index + 1, first_present(beat, &["Beat"]));
        println!();
        println!("| Prompt | Value |");
        println!("|---|---|");
        println!("| Scene | {} |", value(beat, "Scene"));
        println!("| Player action | {} |", value(beat, "Player action"));
        println!("| Aha | {} |", value(beat, "Aha"));
        println!("| Check | {} |", value(beat, "Check"));
        println!("| Behavior probe | {} |", value(beat, "Behavior probe"));
        println!("| Target min | {} |", value(beat, "Target min"));
        println!("| Hint at min | {} |", value(beat, "Hint at min"));
        println!("| Slow max min | {} |", value(beat, "Slow max min"));
        println!("| Mechanism | {} |", value(beat, "Mechanism"));
        println!("| Reliability risk | {} |", value(beat, "Reliability risk"));
        println!("| DC | {} |", value(beat, "DC"));
        println!("| Success | {} |", value(beat, "Success"));
        println!("| Partial | {} |", value(beat, "Partial"));
        println!(
            "| Stall/hint trigger | {} |",
            value(beat, "Stall/hint trigger")
        );
        println!("| Reset effect | {} |", value(beat, "Reset effect"));
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

fn optimize_room(args: OptimizeArgs) -> Result<(), String> {
    let scenes_path = args.room.join("SCENES.md");
    let scenes = fs::read_to_string(&scenes_path)
        .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
    let beat_table = extract_table(&scenes, "## Beat cards")
        .ok_or_else(|| "SCENES.md does not contain a Beat cards table".to_string())?;

    if beat_table.rows.is_empty() {
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
    println!("| Beat count | {} |", beat_table.rows.len());
    println!();
    println!("## Beat Timing");
    println!();
    println!("| # | Beat | Scene | Target min | Hint at min | Slow max min | Pressure | Recommendation |");
    println!("|---:|---|---|---:|---:|---:|---|---|");

    for (index, beat) in beat_table.rows.iter().enumerate() {
        let name = value(beat, "Beat");
        let scene = value(beat, "Scene");
        let target = parse_minutes_field(beat, &["Target min", "Target", "Expected min"]);
        let hint_at = parse_minutes_field(beat, &["Hint at min", "Hint min", "Hint trigger min"]);
        let slow = parse_minutes_field(beat, &["Slow max min", "Max min", "Slow min"]);

        if target.is_none() {
            missing_timing.push(name.clone());
        }

        let target_value = target.unwrap_or(0);
        let slow_value = slow.unwrap_or(target_value);
        total_target += target_value;
        total_slow += slow_value;
        if target.is_some() {
            timings.push(BeatTiming {
                name: name.clone(),
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

        let recommendation = match (target, hint_at, slow) {
            (None, _, _) => "add timing budget",
            (Some(target), Some(hint), _) if hint >= target => {
                late_hint.push(name.clone());
                "move hint before target time"
            }
            (Some(_), None, _) => "add hint trigger minute",
            (Some(target), _, Some(max)) if max >= target + 3 => "add acceleration path",
            _ => "ok",
        };

        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            index + 1,
            name,
            scene,
            display_minutes(target),
            display_minutes(hint_at),
            display_minutes(slow),
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

fn analyze_run(args: AnalyzeArgs) -> Result<(), String> {
    let scenes_path = args.room.join("SCENES.md");
    let scenes = fs::read_to_string(&scenes_path)
        .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
    let beat_table = extract_table(&scenes, "## Beat cards")
        .ok_or_else(|| "SCENES.md does not contain a Beat cards table".to_string())?;

    if beat_table.rows.is_empty() {
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

    for beat in &beat_table.rows {
        let name = value(beat, "Beat");
        let target = parse_minutes_field(beat, &["Target min", "Target", "Expected min"]);
        let hint_at = parse_minutes_field(beat, &["Hint at min", "Hint min", "Hint trigger min"]);
        let slow = parse_minutes_field(beat, &["Slow max min", "Max min", "Slow min"]);
        let actual = actual_for_beat(&args.actual_times, &name);

        if let Some(target) = target {
            target_total += target;
        }

        let status = match (actual, target, hint_at, slow) {
            (None, _, _, _) => {
                missing_actuals.push(name.clone());
                "missing actual"
            }
            (Some(actual), _, _, Some(slow)) if actual > slow => {
                over_slow.push(name.clone());
                actual_total += actual;
                "over slow max"
            }
            (Some(actual), Some(target), Some(hint), _) if actual > target && actual >= hint => {
                hint_pressure.push(name.clone());
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

        let delta = match (actual, target) {
            (Some(actual), Some(target)) => actual as i32 - target as i32,
            _ => 0,
        };

        println!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            name,
            display_minutes(target),
            display_minutes(actual),
            delta,
            display_minutes(hint_at),
            display_minutes(slow),
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

fn record_run(args: RecordArgs) -> Result<(), String> {
    let scenes_path = args.room.join("SCENES.md");
    let scenes = fs::read_to_string(&scenes_path)
        .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
    let beat_table = extract_table(&scenes, "## Beat cards")
        .ok_or_else(|| "SCENES.md does not contain a Beat cards table".to_string())?;
    if beat_table.rows.is_empty() {
        return Err("no beat rows found in SCENES.md".to_string());
    }

    let playtest_path = args.room.join("PLAYTEST.md");
    let playtest = fs::read_to_string(&playtest_path)
        .map_err(|err| format!("failed to read {}: {err}", playtest_path.display()))?;

    let mut rows = Vec::new();
    let mut actual_total = 0;
    let mut missing_actuals = Vec::new();

    for beat in &beat_table.rows {
        let name = value(beat, "Beat");
        let target = parse_minutes_field(beat, &["Target min", "Target", "Expected min"]);
        let hint_at = parse_minutes_field(beat, &["Hint at min", "Hint min", "Hint trigger min"]);
        let slow = parse_minutes_field(beat, &["Slow max min", "Max min", "Slow min"]);
        let actual = actual_for_beat(&args.actual_times, &name);
        if let Some(actual) = actual {
            actual_total += actual;
        } else {
            missing_actuals.push(name.clone());
        }

        rows.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            md_cell(&args.run_id),
            md_cell(&name),
            display_minutes(target),
            display_minutes(actual),
            display_minutes(hint_at),
            display_minutes(hint_used_at(actual, hint_at)),
            display_minutes(slow),
            timing_status(actual, target, hint_at, slow)
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
    fs::write(&playtest_path, updated)
        .map_err(|err| format!("failed to write {}: {err}", playtest_path.display()))?;

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

fn matrix_room(args: MatrixArgs) -> Result<(), String> {
    let scenes_path = args.room.join("SCENES.md");
    let scenes = fs::read_to_string(&scenes_path)
        .map_err(|err| format!("failed to read {}: {err}", scenes_path.display()))?;
    let team_table = extract_table(&scenes, "## Team archetype probes")
        .ok_or_else(|| "SCENES.md does not contain a Team archetype probes table".to_string())?;
    let behavior_table = extract_table(&scenes, "## Behavior probes");

    if team_table.rows.is_empty() {
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
    println!("| Team passes | {} |", team_table.rows.len());
    println!();
    println!("## Required Team Passes");
    println!();
    println!("| # | Team archetype | Probe | Observable signal | Suggested behavior focus |");
    println!("|---:|---|---|---|---|");

    for (index, team) in team_table.rows.iter().enumerate() {
        let team_name = value(team, "Team archetype");
        let behavior_focus = behavior_for_team(behavior_table.as_ref(), &team_name);
        println!(
            "| {} | {} | {} | {} | {} |",
            index + 1,
            team_name,
            value(team, "Scene/beat probe"),
            value(team, "Observable signal"),
            behavior_focus
        );
    }

    println!();
    println!("## Run Commands");
    println!();
    for team in &team_table.rows {
        let team_name = value(team, "Team archetype");
        let behavior_focus = behavior_for_team(behavior_table.as_ref(), &team_name);
        println!(
            "- `cargo run --manifest-path tools\\amaze-harness\\Cargo.toml -- run --room {} --team \"{}\" --behavior \"{}\" --clock {}`",
            args.room.display(),
            team_name,
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

fn inventory_catalog(args: InventoryArgs) -> Result<(), String> {
    let markdown = fs::read_to_string(&args.catalog)
        .map_err(|err| format!("failed to read {}: {err}", args.catalog.display()))?;
    let query = args.query.as_ref().map(|raw| raw.to_lowercase());
    let mut rows = inventory_rows(&markdown);

    if let Some(query) = query {
        rows.retain(|row| {
            row.values()
                .any(|cell| cell.to_lowercase().contains(query.as_str()))
        });
    }

    println!("# AMAZE Inventory");
    println!();
    println!("Catalog: {}", args.catalog.display());
    if let Some(query) = &args.query {
        println!("Query: {query}");
    }
    println!("Items: {}", rows.len());
    println!();
    println!("| ID | Item | Common sources | Unit cost band | Durability | Typical use |");
    println!("|---|---|---|---:|---|---|");
    for row in rows {
        println!(
            "| {} | {} | {} | {} | {} | {} |",
            value(&row, "ID"),
            value(&row, "Item"),
            value(&row, "Common sources"),
            value(&row, "Unit cost band"),
            value(&row, "Durability"),
            value(&row, "Typical use")
        );
    }

    Ok(())
}

fn acceleration_candidates(timings: &[BeatTiming], needed_minutes: u32) -> Vec<String> {
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

fn inventory_rows(markdown: &str) -> Vec<HashMap<String, String>> {
    let mut rows = Vec::new();
    let mut table_lines = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('|') {
            table_lines.push(trimmed.to_string());
            continue;
        }

        if let Some(table) = parse_table(&table_lines) {
            rows.extend(
                table
                    .rows
                    .into_iter()
                    .filter(|row| row.contains_key("ID") && row.contains_key("Item")),
            );
        }
        table_lines.clear();
    }

    if let Some(table) = parse_table(&table_lines) {
        rows.extend(
            table
                .rows
                .into_iter()
                .filter(|row| row.contains_key("ID") && row.contains_key("Item")),
        );
    }

    rows
}

fn extract_table(markdown: &str, heading: &str) -> Option<Table> {
    let mut in_section = false;
    let mut lines = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed == heading {
            in_section = true;
            continue;
        }
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        if in_section && trimmed.starts_with('|') {
            lines.push(trimmed.to_string());
        }
    }

    parse_table(&lines)
}

fn parse_table(lines: &[String]) -> Option<Table> {
    if lines.len() < 2 {
        return None;
    }

    let headers = split_row(&lines[0]);
    if headers.is_empty() {
        return None;
    }

    let rows = lines
        .iter()
        .skip(2)
        .map(|line| split_row(line))
        .filter(|cells| !cells.is_empty())
        .map(|cells| {
            headers
                .iter()
                .cloned()
                .zip(cells)
                .collect::<HashMap<String, String>>()
        })
        .collect();

    Some(Table { rows })
}

fn split_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn value(row: &HashMap<String, String>, key: &str) -> String {
    row.get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .unwrap_or_else(|| "TBD".to_string())
}

fn parse_minutes_field(row: &HashMap<String, String>, keys: &[&str]) -> Option<u32> {
    keys.iter()
        .find_map(|key| row.get(*key))
        .and_then(|raw| parse_minutes(raw))
}

fn parse_minutes(raw: &str) -> Option<u32> {
    let digits: String = raw.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

fn parse_actual_times(raw: &str) -> Result<HashMap<String, u32>, String> {
    let mut times = HashMap::new();
    for pair in raw.split(',') {
        let trimmed = pair.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (beat, minutes) = trimmed
            .split_once('=')
            .ok_or_else(|| format!("invalid actual timing '{trimmed}', expected Beat=minutes"))?;
        let beat = beat.trim();
        if beat.is_empty() {
            return Err(format!("invalid actual timing '{trimmed}', beat is empty"));
        }
        let minutes = minutes
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("invalid minutes in actual timing '{trimmed}'"))?;
        times.insert(beat.to_string(), minutes);
    }
    if times.is_empty() {
        Err("--actual did not contain any timings".to_string())
    } else {
        Ok(times)
    }
}

fn parse_u32_arg(value: Option<&String>, flag: &str) -> Result<u32, String> {
    let raw = value.ok_or_else(|| format!("{flag} requires a number"))?;
    raw.parse::<u32>()
        .map_err(|_| format!("invalid {flag} value '{raw}'"))
}

fn actual_for_beat(actual_times: &HashMap<String, u32>, beat_name: &str) -> Option<u32> {
    actual_times.get(beat_name).copied().or_else(|| {
        beat_name
            .split_whitespace()
            .next()
            .and_then(|id| actual_times.get(id).copied())
    })
}

fn md_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn display_optional_u32(value: Option<u32>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "TBD".to_string())
}

fn timing_status(
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

fn hint_used_at(actual: Option<u32>, hint_at: Option<u32>) -> Option<u32> {
    match (actual, hint_at) {
        (Some(actual), Some(hint)) if actual >= hint => Some(hint),
        _ => None,
    }
}

fn append_rows_to_section(
    markdown: &str,
    heading: &str,
    rows: &[String],
) -> Result<String, String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let heading_index = lines
        .iter()
        .position(|line| line.trim() == heading)
        .ok_or_else(|| format!("missing section heading: {heading}"))?;

    let section_end = lines
        .iter()
        .enumerate()
        .skip(heading_index + 1)
        .find(|(_, line)| line.trim().starts_with("## "))
        .map(|(index, _)| index)
        .unwrap_or(lines.len());

    let mut output = Vec::new();
    output.extend(lines[..section_end].iter().map(|line| line.to_string()));
    output.extend(rows.iter().cloned());
    if section_end < lines.len() {
        output.extend(lines[section_end..].iter().map(|line| line.to_string()));
    }

    Ok(format!("{}\n", output.join("\n")))
}

fn behavior_for_team(behavior_table: Option<&Table>, team_name: &str) -> String {
    let Some(table) = behavior_table else {
        return "not specified".to_string();
    };
    let team_lower = team_name.to_lowercase();
    table
        .rows
        .iter()
        .find(|row| {
            value(row, "Team/persona")
                .to_lowercase()
                .contains(&team_lower)
        })
        .map(|row| value(row, "Behavior"))
        .unwrap_or_else(|| "not specified".to_string())
}

fn display_minutes(value: Option<u32>) -> String {
    value
        .map(|minutes| minutes.to_string())
        .unwrap_or_else(|| "TBD".to_string())
}

fn fit_status(slack: i32) -> &'static str {
    if slack < 0 {
        "over target"
    } else if slack < 3 {
        "tight"
    } else {
        "fits"
    }
}

fn slack_status(slack: i32) -> &'static str {
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

fn delta_status(delta: i32) -> &'static str {
    if delta > 0 {
        "over"
    } else if delta < 0 {
        "under"
    } else {
        "on target"
    }
}

fn first_present(row: &HashMap<String, String>, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| row.get(*key).filter(|value| !value.is_empty()).cloned())
        .unwrap_or_else(|| "TBD".to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_actual_times;

    #[test]
    fn parses_actual_times() {
        let times = parse_actual_times("P1=4,P2=5").expect("valid timings");
        assert_eq!(times.get("P1"), Some(&4));
        assert_eq!(times.get("P2"), Some(&5));
    }

    #[test]
    fn rejects_invalid_actual_times() {
        assert!(parse_actual_times("P1:4").is_err());
    }

    #[test]
    fn matches_actual_by_beat_id_prefix() {
        let times = parse_actual_times("P1=4").expect("valid timings");
        assert_eq!(
            super::actual_for_beat(&times, "P1 route fragments"),
            Some(4)
        );
    }

    #[test]
    fn finds_behavior_for_team() {
        let mut row = std::collections::HashMap::new();
        row.insert("Behavior".to_string(), "brute force".to_string());
        row.insert("Team/persona".to_string(), "Speed team".to_string());
        let table = super::Table { rows: vec![row] };
        assert_eq!(
            super::behavior_for_team(Some(&table), "Speed team"),
            "brute force"
        );
    }

    #[test]
    fn appends_rows_to_section() {
        let markdown =
            "# Playtest\n\n## Beat timing evidence\n\n| A | B |\n|---|---|\n\n## Scores\n";
        let updated = super::append_rows_to_section(
            markdown,
            "## Beat timing evidence",
            &["| x | y |".to_string()],
        )
        .expect("section exists");
        assert!(updated.contains("| x | y |\n## Scores"));
    }

    #[test]
    fn escapes_markdown_cells() {
        assert_eq!(super::md_cell("a|b\nc"), "a\\|b c");
    }

    #[test]
    fn extracts_inventory_rows_from_multiple_tables() {
        let markdown = "\
# Catalog

## A

| ID | Item | Unit cost band |
|---|---|---:|
| MECH-001 | latch | `$5-25` |

## Notes

| Not ID | Item |
|---|---|
| x | ignored |

## B

| ID | Item | Unit cost band |
|---|---|---:|
| ELEC-001 | LED strip | `$10-35` |
";
        let rows = super::inventory_rows(markdown);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("ID").map(String::as_str), Some("MECH-001"));
        assert_eq!(rows[1].get("Item").map(String::as_str), Some("LED strip"));
    }
}
