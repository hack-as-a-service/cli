#[macro_use]
extern crate lazy_static;

mod commands;
mod credentials;
mod models;
mod utils;

use clap::{App, AppSettings, Arg};
use std::process;

use commands::{apps, auth, deploy::deploy_command, postgres::postgres_command};

fn main() {
	let app = App::new("Hack as a Service")
		.bin_name("haas")
		.version("1.0")
		.author("HaaS Development Team (https://github.com/hack-as-a-service)")
		.about("CLI for Hack as a Service")
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(
			App::new("deploy")
				.about("Deploy an app to HaaS")
				.alias("d")
				.arg(
					Arg::new("app")
						.takes_value(true)
						.value_name("app")
						.long("app")
						.short('a')
						.required(true)
						.help("App to deploy to"),
				)
				.arg(
					Arg::new("detach")
						.short('d')
						.long("detach")
						.help("Detach from the shell after starting deployment"),
				),
		)
		.subcommand(
			App::new("postgres")
				.alias("pg")
				.about("Connect to an app's PostgreSQL database")
				.arg(
					Arg::new("app")
						.takes_value(true)
						.value_name("app")
						.long("app")
						.short('a')
						.required(true)
						.help("App to connect to"),
				),
		)
		.subcommand(
			App::new("auth")
				.about("Manage authentication")
				.setting(AppSettings::SubcommandRequiredElseHelp)
				.subcommand(App::new("login").about("Log in to HaaS"))
				.subcommand(App::new("info").alias("test").about("Test authentication")),
		)
		.subcommand(
			App::new("apps").about("Manage apps").arg(
				Arg::new("team")
					.takes_value(true)
					.value_name("team")
					.help("List apps for the given team")
					.long("team")
					.short('t'),
			),
		);

	let matches = app.get_matches();

	let result: Result<(), String> = match matches.subcommand() {
		Some(("deploy", matches)) => deploy_command(matches),
		Some(("postgres", matches)) => postgres_command(matches),
		Some(("apps", matches)) => apps::apps_command(matches),
		Some(("auth", matches)) => match matches.subcommand() {
			Some(("login", matches)) => auth::login_command(matches),
			Some(("info", matches)) => auth::info_command(matches),
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
