use std::env;
use std::process;

mod cli;
mod commands;
mod domain;
mod evidence;
mod markdown;
mod room;
mod rules;
mod scenes;
mod sim;

use cli::{parse_cli, print_help};

fn main() {
    let cli = parse_cli(env::args().skip(1).collect()).unwrap_or_else(|err| {
        eprintln!("error: {err}\n");
        print_help();
        process::exit(2);
    });

    if let Err(err) = commands::run(cli.command) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
