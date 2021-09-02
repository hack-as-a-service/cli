// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
use clap::{App, AppSettings, Arg, SubCommand};
use commands::postgres::postgres_command;

use crate::commands::deploy::deploy_command;

mod commands;

fn main() {
    let app = App::new("Hack as a Service")
        .bin_name("haas")
        .version("1.0")
        .author("HaaS Development Team (https://github.com/hack-as-a-service)")
        .about("CLI for Hack as a Service")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploy an app to HaaS")
                .alias("d")
                .arg(
                    Arg::with_name("app")
                        .takes_value(true)
                        .value_name("app")
                        .long("app")
                        .short("a")
                        .required(true)
                        .help("App to deploy to"),
                )
                .arg(
                    Arg::with_name("detach")
                        .short("d")
                        .long("detach")
                        .help("Detach from the shell after starting deployment"),
                ),
        )
        .subcommand(
            SubCommand::with_name("postgres")
                .alias("pg")
                .about("Connect to an app's PostgreSQL database")
                .arg(
                    Arg::with_name("app")
                        .takes_value(true)
                        .value_name("app")
                        .long("app")
                        .short("a")
                        .required(true)
                        .help("App to connect to"),
                ),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("deploy", Some(matches)) => deploy_command(matches),
        ("postgres", Some(matches)) => postgres_command(matches),
        _ => unreachable!(),
    }
}
