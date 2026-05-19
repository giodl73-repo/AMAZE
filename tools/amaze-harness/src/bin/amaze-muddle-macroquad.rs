use amaze_harness::{prism_vault_muddle_host, silverstream_muddle_host};
use macroquad::prelude::*;
use muddle_core::MuddleClientHostRegistration;
use muddle_macroquad::{
    apply_default_macroquad_paths, macroquad_usage, macroquad_window_conf,
    parse_macroquad_run_options, run_muddle_macroquad_hosts, MuddleMacroquadRunConfig,
};

const HOST_NAME: &str = "amaze-silverstream";
const PRISM_HOST_NAME: &str = "amaze-prism-vault";

fn window_conf() -> Conf {
    macroquad_window_conf("AMAZE MUDDLE")
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut options = match parse_macroquad_run_options(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("{}", macroquad_usage());
            std::process::exit(1);
        }
    };
    if options.host_name.is_none() && !options.list_hosts && !options.show_help {
        options.host_name = Some(HOST_NAME.to_string());
    }
    if options.host_name.as_deref() == Some(PRISM_HOST_NAME) {
        apply_default_macroquad_paths(
            &mut options,
            "prism-vault.macroquad.muddle",
            "prism-vault.macroquad.txt",
            "prism-vault.import.muddle",
            "prism-vault.export.muddle",
        );
    } else {
        apply_default_macroquad_paths(
            &mut options,
            "silverstream.macroquad.muddle",
            "silverstream.macroquad.txt",
            "silverstream.import.muddle",
            "silverstream.export.muddle",
        );
    }

    let registrations = vec![MuddleClientHostRegistration {
        name: HOST_NAME,
        category: "Games",
        description: "AMAZE Silverstream: native route-to-radio escape-room slice.",
        suggested_commands:
            "`go route`, `sort postcards`, `set breakers`, `sort galley`, `align table`, `tune radio`, `unlock hatch`, `go hatch`.",
        create: || Box::new(silverstream_muddle_host()),
    },
    MuddleClientHostRegistration {
        name: PRISM_HOST_NAME,
        category: "Games",
        description: "AMAZE Prism Vault: native light-and-mirror escape-room slice.",
        suggested_commands:
            "`go lens`, `align lens`, `mix color`, `set mirrors`, `unlock vault`, `go exit`.",
        create: || Box::new(prism_vault_muddle_host()),
    }];

    if let Err(error) = run_muddle_macroquad_hosts(
        registrations,
        options,
        MuddleMacroquadRunConfig {
            screen_title: "AMAZE MUDDLE".to_string(),
        },
    )
    .await
    {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
