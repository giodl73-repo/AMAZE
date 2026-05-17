use amaze_harness::silverstream_muddle_host;
use muddle_cli::{run_muddle_host, MuddleCliHostInfo};

fn main() -> std::io::Result<()> {
    let mut host = silverstream_muddle_host();
    run_muddle_host(
        &mut host,
        MuddleCliHostInfo {
            name: "amaze-silverstream",
            description: "AMAZE Silverstream: product-owned MUDDLE host.",
            suggested_commands:
                "`look`, `go receiver`, `inspect clue`, `tune signal`, `unlock hatch`, `go hatch`, `quit`.",
        },
    )
    .map(|_| ())
}
