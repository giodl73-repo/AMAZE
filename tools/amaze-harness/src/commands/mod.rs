pub(crate) mod bench;
pub(crate) mod catalog;
pub(crate) mod check;
pub(crate) mod lifecycle;
pub(crate) mod lint;
pub(crate) mod ops;
pub(crate) mod score;
pub(crate) mod simulation;
pub(crate) mod timing;
pub(crate) mod visuals;

use crate::cli::Command;

pub(crate) fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Run(args) => timing::run_room(args),
        Command::Simulate(args) => simulation::simulate_room(args),
        Command::Optimize(args) => timing::optimize_room(args),
        Command::Analyze(args) => timing::analyze_run(args),
        Command::Record(args) => timing::record_run(args),
        Command::Matrix(args) => timing::matrix_room(args),
        Command::Score(args) => score::score_room_pack(args),
        Command::Inventory(args) => catalog::inventory_catalog(args),
        Command::Packages(args) => catalog::packages_catalog(args),
        Command::Check { room } => check::check_room(&room),
        Command::Lint(args) => lint::lint_room_pack(args),
        Command::Visuals(args) => visuals::visual_readiness_report(args),
        Command::Bench(args) => bench::bench_evidence_report(args),
        Command::Ops(args) => ops::ops_readiness_report(args),
        Command::Lifecycle(args) => lifecycle::lifecycle_report(args),
        Command::Help => {
            crate::cli::print_help();
            Ok(())
        }
    }
}
