#[macro_use]
extern crate lazy_static;

mod commands;
mod credentials;
mod models;
mod utils;

use clap::{Parser, Subcommand};
use std::process;

use commands::{
	apps,
	auth::{self, AuthCommand},
	deploy::deploy_command,
	postgres::postgres_command,
};

#[derive(Parser, Debug)]
#[clap(
	name = "Hack as a Service",
	author = "HaaS Development Team",
	about = "CLI for Hack as a Service",
	bin_name = "haas"
)]
struct Cli {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
	/// Manage apps
	Apps {
		/// List apps for the given team
		#[clap(long, short)]
		team: Option<String>,
	},

	/// Manage authentication
	Auth {
		#[clap(subcommand)]
		command: AuthCommand,
	},

	/// Deploy an app to HaaS
	Deploy {
		/// App to deploy to
		#[clap(long, short)]
		app: String,

		/// Detach from the shell after starting deployment
		#[clap(long, short)]
		detach: bool,
	},

	/// Connect to an app's PostgreSQL database
	Postgres {
		/// App to connect to
		#[clap(long, short)]
		app: String,
	},
}

fn main() {
	let cli = Cli::parse();

	let result = match cli.command {
		Command::Apps { team } => apps::apps_command(team),
		Command::Auth { command } => match command {
			AuthCommand::Login {} => auth::login_command(),
			AuthCommand::Info {} => auth::info_command(),
		},
		Command::Deploy { app, detach } => deploy_command(app, detach),
		Command::Postgres { app } => postgres_command(app),
	};

	match result {
		Ok(_) => (),
		Err(x) => {
			println!("{}", x);
			process::exit(1)
		}
	}
}
