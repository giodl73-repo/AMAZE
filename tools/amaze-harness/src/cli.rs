use std::collections::HashMap;
use std::path::PathBuf;

use crate::evidence::normalize_evidence_status;

const DEFAULT_TARGET_MINUTES: u32 = 45;

#[derive(Debug)]
pub(crate) struct Cli {
    pub(crate) command: Command,
}

#[derive(Debug)]
pub(crate) enum Command {
    Run(RunArgs),
    Simulate(SimulateArgs),
    Optimize(OptimizeArgs),
    Analyze(AnalyzeArgs),
    Record(RecordArgs),
    Matrix(MatrixArgs),
    Score(ScoreArgs),
    Inventory(InventoryArgs),
    Packages(PackagesArgs),
    Check { room: PathBuf },
    Lint(LintArgs),
    Visuals(VisualsArgs),
    Bench(BenchArgs),
    Ops(OpsArgs),
    Help,
}

#[derive(Debug)]
pub(crate) struct RunArgs {
    pub(crate) room: PathBuf,
    pub(crate) team: String,
    pub(crate) behavior: Option<String>,
    pub(crate) clock_minutes: Option<u32>,
}

#[derive(Debug)]
pub(crate) struct SimulateArgs {
    pub(crate) room: PathBuf,
    pub(crate) runs: u32,
    pub(crate) seed: u64,
    pub(crate) target_minutes: u32,
}

#[derive(Debug)]
pub(crate) struct OptimizeArgs {
    pub(crate) room: PathBuf,
    pub(crate) target_minutes: u32,
}

#[derive(Debug)]
pub(crate) struct AnalyzeArgs {
    pub(crate) room: PathBuf,
    pub(crate) team: Option<String>,
    pub(crate) target_minutes: u32,
    pub(crate) actual_times: HashMap<String, u32>,
}

#[derive(Debug)]
pub(crate) struct RecordArgs {
    pub(crate) room: PathBuf,
    pub(crate) run_id: String,
    pub(crate) version: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) run_type: Option<String>,
    pub(crate) team: Option<String>,
    pub(crate) personas: Option<String>,
    pub(crate) players: Option<u32>,
    pub(crate) finish: Option<u32>,
    pub(crate) hints: Option<u32>,
    pub(crate) stuck: Option<String>,
    pub(crate) failures: Option<String>,
    pub(crate) safety: Option<String>,
    pub(crate) reset: Option<u32>,
    pub(crate) memorable: Option<String>,
    pub(crate) target_minutes: u32,
    pub(crate) actual_times: HashMap<String, u32>,
}

#[derive(Debug)]
pub(crate) struct MatrixArgs {
    pub(crate) room: PathBuf,
    pub(crate) target_minutes: u32,
}

#[derive(Debug)]
pub(crate) struct ScoreArgs {
    pub(crate) room: Option<PathBuf>,
    pub(crate) rooms: Option<PathBuf>,
    pub(crate) target_minutes: Option<u32>,
}

#[derive(Debug)]
pub(crate) struct InventoryArgs {
    pub(crate) catalog: PathBuf,
    pub(crate) query: Option<String>,
}

#[derive(Debug)]
pub(crate) struct PackagesArgs {
    pub(crate) catalog: PathBuf,
    pub(crate) query: Option<String>,
}

#[derive(Debug)]
pub(crate) struct LintArgs {
    pub(crate) room: Option<PathBuf>,
    pub(crate) rooms: Option<PathBuf>,
}

#[derive(Debug)]
pub(crate) struct VisualsArgs {
    pub(crate) room: Option<PathBuf>,
    pub(crate) rooms: Option<PathBuf>,
    pub(crate) open_c5: bool,
    pub(crate) status: Option<String>,
    pub(crate) gap: Option<String>,
}

#[derive(Debug)]
pub(crate) struct BenchArgs {
    pub(crate) room: Option<PathBuf>,
    pub(crate) rooms: Option<PathBuf>,
    pub(crate) open: bool,
    pub(crate) status: Option<String>,
    pub(crate) kind: Option<String>,
    pub(crate) blocker: Option<String>,
    pub(crate) target: Option<String>,
}

#[derive(Debug)]
pub(crate) struct OpsArgs {
    pub(crate) room: Option<PathBuf>,
    pub(crate) rooms: Option<PathBuf>,
    pub(crate) stagger_minutes: u32,
    pub(crate) finale_minutes: u32,
}

pub(crate) fn parse_cli(args: Vec<String>) -> Result<Cli, String> {
    if args.is_empty() {
        return Ok(Cli {
            command: Command::Help,
        });
    }

    match args[0].as_str() {
        "run" => parse_run(&args[1..]).map(|command| Cli { command }),
        "simulate" | "sim" => parse_simulate(&args[1..]).map(|command| Cli { command }),
        "optimize" => parse_optimize(&args[1..]).map(|command| Cli { command }),
        "analyze" => parse_analyze(&args[1..]).map(|command| Cli { command }),
        "record" => parse_record(&args[1..]).map(|command| Cli { command }),
        "matrix" => parse_matrix(&args[1..]).map(|command| Cli { command }),
        "score" => parse_score(&args[1..]).map(|command| Cli { command }),
        "inventory" => parse_inventory(&args[1..]).map(|command| Cli { command }),
        "packages" => parse_packages(&args[1..]).map(|command| Cli { command }),
        "check" => parse_check(&args[1..]).map(|command| Cli { command }),
        "lint" => parse_lint(&args[1..]).map(|command| Cli { command }),
        "visuals" => parse_visuals(&args[1..]).map(|command| Cli { command }),
        "bench" => parse_bench(&args[1..]).map(|command| Cli { command }),
        "ops" => parse_ops(&args[1..]).map(|command| Cli { command }),
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
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
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

fn parse_score(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut rooms = None;
    let mut target_minutes = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--rooms" => {
                i += 1;
                rooms = args.get(i).map(PathBuf::from);
            }
            "--target" => {
                i += 1;
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
            }
            other => return Err(format!("unknown score argument '{other}'")),
        }
        i += 1;
    }

    if room.is_none() && rooms.is_none() {
        return Err("score requires --room <path> or --rooms <rooms-root>".to_string());
    }
    if room.is_some() && rooms.is_some() {
        return Err("score accepts either --room or --rooms, not both".to_string());
    }

    Ok(Command::Score(ScoreArgs {
        room,
        rooms,
        target_minutes,
    }))
}

fn parse_simulate(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut runs = Some(100);
    let mut seed = Some(1);
    let mut target_minutes = Some(DEFAULT_TARGET_MINUTES);
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--runs" => {
                i += 1;
                runs = Some(parse_u32_arg(args.get(i), "--runs")?);
            }
            "--seed" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--seed requires a number".to_string())?;
                seed = Some(
                    raw.parse::<u64>()
                        .map_err(|_| format!("invalid --seed value '{raw}'"))?,
                );
            }
            "--target" => {
                i += 1;
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
            }
            other => return Err(format!("unknown simulate argument '{other}'")),
        }
        i += 1;
    }

    let runs = runs.unwrap_or(100);
    if runs == 0 {
        return Err("--runs must be greater than 0".to_string());
    }

    Ok(Command::Simulate(SimulateArgs {
        room: room.ok_or_else(|| "simulate requires --room <path>".to_string())?,
        runs,
        seed: seed.unwrap_or(1),
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
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
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
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
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
                target_minutes = Some(parse_u32_arg(args.get(i), "--target")?);
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

fn parse_packages(args: &[String]) -> Result<Command, String> {
    let mut catalog = PathBuf::from("components\\BEAT-PACKAGES.md");
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
            other => return Err(format!("unknown packages argument '{other}'")),
        }
        i += 1;
    }

    Ok(Command::Packages(PackagesArgs { catalog, query }))
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
                clock_minutes = Some(parse_u32_arg(args.get(i), "--clock")?);
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

fn parse_lint(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut rooms = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--rooms" => {
                i += 1;
                rooms = args.get(i).map(PathBuf::from);
            }
            other => return Err(format!("unknown lint argument '{other}'")),
        }
        i += 1;
    }

    if room.is_none() && rooms.is_none() {
        return Err("lint requires --room <path> or --rooms <rooms-root>".to_string());
    }
    if room.is_some() && rooms.is_some() {
        return Err("lint accepts either --room or --rooms, not both".to_string());
    }

    Ok(Command::Lint(LintArgs { room, rooms }))
}

fn parse_visuals(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut rooms = None;
    let mut open_c5 = false;
    let mut status = None;
    let mut gap = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--rooms" => {
                i += 1;
                rooms = args.get(i).map(PathBuf::from);
            }
            "--open-c5" => {
                open_c5 = true;
            }
            "--status" => {
                i += 1;
                let value = args.get(i).ok_or_else(|| {
                    "--status requires build-ready, draft, or blocked".to_string()
                })?;
                if !matches!(value.as_str(), "build-ready" | "draft" | "blocked") {
                    return Err(format!(
                        "invalid --status value '{value}' (use build-ready, draft, or blocked)"
                    ));
                }
                status = Some(value.clone());
            }
            "--gap" => {
                i += 1;
                gap = Some(
                    args.get(i)
                        .ok_or_else(|| "--gap requires text".to_string())?
                        .clone(),
                );
            }
            other => return Err(format!("unknown visuals argument '{other}'")),
        }
        i += 1;
    }

    if room.is_none() && rooms.is_none() {
        return Err("visuals requires --room <path> or --rooms <rooms-root>".to_string());
    }
    if room.is_some() && rooms.is_some() {
        return Err("visuals accepts either --room or --rooms, not both".to_string());
    }

    Ok(Command::Visuals(VisualsArgs {
        room,
        rooms,
        open_c5,
        status,
        gap,
    }))
}

fn parse_bench(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut rooms = None;
    let mut open = false;
    let mut status = None;
    let mut kind = None;
    let mut blocker = None;
    let mut target = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--rooms" => {
                i += 1;
                rooms = args.get(i).map(PathBuf::from);
            }
            "--open" => open = true,
            "--status" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--status requires a value".to_string())?;
                status = Some(normalize_evidence_status(raw));
            }
            "--kind" => {
                i += 1;
                let raw = args
                    .get(i)
                    .ok_or_else(|| "--kind requires a value".to_string())?;
                let normalized = raw.to_lowercase();
                if !matches!(normalized.as_str(), "bench" | "admin" | "chaos") {
                    return Err("--kind must be bench, admin, or chaos".to_string());
                }
                kind = Some(normalized);
            }
            "--blocker" => {
                i += 1;
                blocker = Some(
                    args.get(i)
                        .ok_or_else(|| "--blocker requires text".to_string())?
                        .to_lowercase(),
                );
            }
            "--target" => {
                i += 1;
                target = Some(
                    args.get(i)
                        .ok_or_else(|| "--target requires text".to_string())?
                        .to_lowercase(),
                );
            }
            other => return Err(format!("unknown bench argument '{other}'")),
        }
        i += 1;
    }

    if room.is_none() && rooms.is_none() {
        return Err("bench requires --room <path> or --rooms <rooms-root>".to_string());
    }
    if room.is_some() && rooms.is_some() {
        return Err("bench accepts either --room or --rooms, not both".to_string());
    }

    Ok(Command::Bench(BenchArgs {
        room,
        rooms,
        open,
        status,
        kind,
        blocker,
        target,
    }))
}

fn parse_ops(args: &[String]) -> Result<Command, String> {
    let mut room = None;
    let mut rooms = None;
    let mut stagger_minutes = Some(10);
    let mut finale_minutes = Some(10);
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--room" => {
                i += 1;
                room = args.get(i).map(PathBuf::from);
            }
            "--rooms" => {
                i += 1;
                rooms = args.get(i).map(PathBuf::from);
            }
            "--stagger" => {
                i += 1;
                stagger_minutes = Some(parse_u32_arg(args.get(i), "--stagger")?);
            }
            "--finale-window" => {
                i += 1;
                finale_minutes = Some(parse_u32_arg(args.get(i), "--finale-window")?);
            }
            other => return Err(format!("unknown ops argument '{other}'")),
        }
        i += 1;
    }

    if room.is_none() && rooms.is_none() {
        return Err("ops requires --room <path> or --rooms <rooms-root>".to_string());
    }
    if room.is_some() && rooms.is_some() {
        return Err("ops accepts either --room or --rooms, not both".to_string());
    }
    if stagger_minutes == Some(0) {
        return Err("--stagger must be greater than 0".to_string());
    }
    if finale_minutes == Some(0) {
        return Err("--finale-window must be greater than 0".to_string());
    }

    Ok(Command::Ops(OpsArgs {
        room,
        rooms,
        stagger_minutes: stagger_minutes.unwrap_or(10),
        finale_minutes: finale_minutes.unwrap_or(10),
    }))
}

pub(crate) fn print_help() {
    println!(
        "AMAZE harness\n\n\
          Commands:\n\
            amaze check --room <room-path>\n\
            amaze lint --room <room-path>\n\
            amaze lint --rooms <rooms-root>\n\
            amaze visuals --room <room-path>\n\
            amaze visuals --rooms <rooms-root>\n\
            amaze visuals --rooms <rooms-root> --open-c5\n\
            amaze visuals --rooms <rooms-root> --status draft --gap pouch\n\
            amaze bench --room <room-path>\n\
            amaze bench --rooms <rooms-root> --open\n\
            amaze bench --rooms <rooms-root> --kind admin --status not-run\n\
            amaze bench --rooms <rooms-root> --open --blocker \"P2 promotion\"\n\
            amaze bench --rooms <rooms-root> --open --target DEV-SOCKET-001\n\
            amaze ops --room <room-path>\n\
            amaze ops --rooms <rooms-root> [--stagger 10] [--finale-window 10]\n\
            amaze run --room <room-path> --team <team archetype> [--behavior <behavior>] [--clock <minutes>]\n\
            amaze simulate --room <room-path> [--runs 100] [--seed 1] [--target <minutes>]\n\
            amaze optimize --room <room-path> [--target <minutes>]\n\
            amaze analyze --room <room-path> --actual P1=4,P2=5 [--team <team>] [--target <minutes>]\n\n\
            amaze record --room <room-path> --run <id> --actual P1=4,P2=5 [--team <team>] [--finish <minutes>] [--hints <count>]\n\
            amaze matrix --room <room-path> [--target <minutes>]\n\n\
            amaze score --room <room-path> [--target <minutes>]\n\
            amaze score --rooms <rooms-root> [--target <minutes>]\n\n\
            amaze inventory [--query <text>] [--catalog <path>]\n\n\
            amaze packages [--query <text>] [--catalog <path>]\n\n\
          Example:\n\
            cargo run --manifest-path tools\\amaze-harness\\Cargo.toml -- lint --rooms rooms"
    );
}

pub(crate) fn parse_actual_times(raw: &str) -> Result<HashMap<String, u32>, String> {
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

#[cfg(test)]
mod tests {
    use super::{parse_actual_times, parse_cli, Command};

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
    fn parses_lint_room_or_rooms() {
        let one_room = parse_cli(vec![
            "lint".to_string(),
            "--room".to_string(),
            "rooms\\TEMPLATE".to_string(),
        ])
        .expect("parse room lint");
        assert!(matches!(one_room.command, Command::Lint(args) if args.room.is_some()));

        let all_rooms = parse_cli(vec![
            "lint".to_string(),
            "--rooms".to_string(),
            "rooms".to_string(),
        ])
        .expect("parse rooms lint");
        assert!(matches!(all_rooms.command, Command::Lint(args) if args.rooms.is_some()));
    }

    #[test]
    fn parses_score_room_or_rooms() {
        let one_room = parse_cli(vec![
            "score".to_string(),
            "--room".to_string(),
            "rooms\\TEMPLATE".to_string(),
        ])
        .expect("parse room score");
        assert!(matches!(one_room.command, Command::Score(args) if args.room.is_some()));

        let all_rooms = parse_cli(vec![
            "score".to_string(),
            "--rooms".to_string(),
            "rooms".to_string(),
            "--target".to_string(),
            "45".to_string(),
        ])
        .expect("parse rooms score");
        assert!(
            matches!(all_rooms.command, Command::Score(args) if args.rooms.is_some() && args.target_minutes == Some(45))
        );
    }

    #[test]
    fn parses_ops_room_or_rooms() {
        let one_room = parse_cli(vec![
            "ops".to_string(),
            "--room".to_string(),
            "rooms\\TEMPLATE".to_string(),
        ])
        .expect("parse room ops");
        assert!(matches!(one_room.command, Command::Ops(args) if args.room.is_some()));

        let all_rooms = parse_cli(vec![
            "ops".to_string(),
            "--rooms".to_string(),
            "rooms".to_string(),
            "--stagger".to_string(),
            "12".to_string(),
            "--finale-window".to_string(),
            "8".to_string(),
        ])
        .expect("parse rooms ops");
        assert!(
            matches!(all_rooms.command, Command::Ops(args) if args.rooms.is_some() && args.stagger_minutes == 12 && args.finale_minutes == 8)
        );
    }
}
