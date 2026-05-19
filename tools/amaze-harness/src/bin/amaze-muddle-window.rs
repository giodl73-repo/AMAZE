use amaze_harness::{prism_vault_muddle_host, silverstream_muddle_host};
use muddle_window::{run_muddle_window_hosts_from_env_args, MuddleWindowHostRegistration};

fn main() -> std::io::Result<()> {
    run_muddle_window_hosts_from_env_args(vec![
        MuddleWindowHostRegistration {
            name: "amaze-silverstream",
            category: "Games",
            description: "AMAZE Silverstream: product-owned MUDDLE window host.",
            suggested_commands:
                "`go route`, `sort postcards`, `set breakers`, `sort galley`, `align table`, `tune radio`, `unlock hatch`, `go hatch`.",
            create: || Box::new(silverstream_muddle_host()),
        },
        MuddleWindowHostRegistration {
            name: "amaze-prism-vault",
            category: "Games",
            description: "AMAZE Prism Vault: product-owned MUDDLE window host.",
            suggested_commands:
                "`go lens`, `align lens`, `mix color`, `set mirrors`, `unlock vault`, `go exit`.",
            create: || Box::new(prism_vault_muddle_host()),
        },
    ])
}
