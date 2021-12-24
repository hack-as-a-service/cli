use clap::ArgMatches;
use termion::style;

use crate::{
	credentials::get_token,
	models::User,
	utils::{stringify_err, stringify_status_err},
};

use reqwest::blocking as reqwest;

pub fn info_command(_matches: &ArgMatches) -> Result<(), String> {
	let token = get_token()?;

	let client = reqwest::Client::new();

	let user = client
		.get("https://haas.hackclub.com/api/users/me")
		.bearer_auth(&token)
		.send()
		.map_err(stringify_err)?
		.error_for_status()
		.map_err(stringify_status_err)?
		.json::<User>()
		.map_err(stringify_err)?;

	println!(
		"👤 {faint}Logged in as{reset} {}
💾 {faint}User ID:{reset} {}
💬 {faint}Slack user ID:{reset} {}",
		user.name,
		user.id,
		user.slack_user_id,
		faint = style::Faint,
		reset = style::Reset
	);

	Ok(())
}
