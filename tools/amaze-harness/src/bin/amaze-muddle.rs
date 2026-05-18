use amaze_harness::silverstream_muddle_host;
use muddle_cli::{run_muddle_host_from_env_args, MuddleCliHostInfo};

fn main() -> std::io::Result<()> {
    let mut host = silverstream_muddle_host();
    run_muddle_host_from_env_args(
        &mut host,
        MuddleCliHostInfo {
            name: "amaze-silverstream",
            description: "AMAZE Silverstream: product-owned MUDDLE host.",
            suggested_commands:
                "`go route`, `sort postcards`, `set breakers`, `sort galley`, `align table`, `tune radio`, `unlock hatch`, `go hatch`, `quit`.",
        },
    )
    .map(|_| ())
}
