use amaze_harness::silverstream_muddle_host;
use muddle_window::{run_muddle_window_hosts_from_env_args, MuddleWindowHostRegistration};

fn main() -> std::io::Result<()> {
    run_muddle_window_hosts_from_env_args(vec![MuddleWindowHostRegistration {
        name: "amaze-silverstream",
        description: "AMAZE Silverstream: product-owned MUDDLE window host.",
        suggested_commands:
            "`look`, `go receiver`, `inspect clue`, `tune signal`, `unlock hatch`, `go hatch`.",
        create: || Box::new(silverstream_muddle_host()),
    }])
}
