#[macro_use]
extern crate lazy_static;

mod commands;
mod credentials;
mod models;
mod utils;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process;

use commands::{auth, deploy::deploy_command, postgres::postgres_command};

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
		)
		.subcommand(
			SubCommand::with_name("auth")
				.about("Manage authentication")
				.setting(AppSettings::SubcommandRequiredElseHelp)
				.subcommand(SubCommand::with_name("login").about("Log in to HaaS"))
				.subcommand(
					SubCommand::with_name("info")
						.alias("test")
						.about("Test authentication"),
				),
		);

	let matches = app.get_matches();

	let result: Result<(), String> = match matches.subcommand() {
		("deploy", Some(matches)) => deploy_command(matches),
		("postgres", Some(matches)) => postgres_command(matches),
		("auth", Some(matches)) => match matches.subcommand() {
			("login", Some(matches)) => auth::login_command(matches),
			("info", Some(matches)) => auth::info_command(matches),
			_ => unreachable!(),
		},
		_ => unreachable!(),
	};

	match result {
		Ok(_) => (),
		Err(x) => {
			println!("{}", x);
			process::exit(1)
		}
	}
}
