use amaze_harness::prism_vault_muddle_host;
use muddle_cli::{run_muddle_host_from_env_args, MuddleCliHostInfo};

fn main() -> std::io::Result<()> {
    let mut host = prism_vault_muddle_host();
    run_muddle_host_from_env_args(
        &mut host,
        MuddleCliHostInfo {
            name: "amaze-prism-vault",
            description: "AMAZE Prism Vault: product-owned MUDDLE host.",
            suggested_commands:
                "`go lens`, `align lens`, `mix color`, `set mirrors`, `unlock vault`, `go exit`, `quit`.",
        },
    )
    .map(|_| ())
}
