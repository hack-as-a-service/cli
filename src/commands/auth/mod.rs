use clap::Subcommand;

mod info;
mod login;

pub use info::*;
pub use login::*;

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
	/// Log in to HaaS
	Login {},

	/// Test authentication
	Info {},
}
