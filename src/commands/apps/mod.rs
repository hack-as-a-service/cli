use clap::ArgMatches;
use reqwest::blocking as reqwest;
use termion::{color, style};

use crate::{
	credentials::get_token,
	models::{App, Team},
	utils::{stringify_err, stringify_status_err},
};

pub fn apps_command(matches: &ArgMatches) -> Result<(), String> {
	let token = get_token()?;

	let client = reqwest::Client::new();

	let team = if let Some(team) = matches.value_of("team") {
		client
			.get(&format!("https://haas.hackclub.com/api/teams/{}", team))
			.bearer_auth(&token)
			.send()
			.map_err(stringify_err)?
			.error_for_status()
			.map_err(|e| {
				if e.status() == Some(::reqwest::StatusCode::NOT_FOUND) {
					format!(
						"Team {bold}{}{reset} not found.",
						team,
						bold = style::Bold,
						reset = style::Reset
					)
				} else {
					stringify_status_err(e)
				}
			})?
			.json::<Team>()
			.map_err(stringify_err)?
	} else {
		let teams = client
			.get("https://haas.hackclub.com/api/users/me/teams")
			.bearer_auth(&token)
			.send()
			.map_err(stringify_err)?
			.error_for_status()
			.map_err(stringify_status_err)?
			.json::<Vec<Team>>()
			.map_err(stringify_err)?;

		teams.into_iter().find(|t| t.personal).unwrap()
	};

	let apps = client
		.get(&format!(
			"https://haas.hackclub.com/api/teams/{}/apps",
			team.slug
		))
		.bearer_auth(&token)
		.send()
		.map_err(stringify_err)?
		.error_for_status()
		.map_err(stringify_status_err)?
		.json::<Vec<App>>()
		.map_err(stringify_err)?;

	if team.personal {
		println!(
			"\n{faint}Your personal apps:{reset}\n",
			faint = style::Faint,
			reset = style::Reset
		);
	} else {
		println!(
			"\n{faint}Apps for team {reset}{}{faint}:{reset}\n",
			team.name.unwrap_or(team.slug),
			faint = style::Faint,
			reset = style::Reset
		);
	}

	for app in apps.iter() {
		let enabled = if app.enabled {
			format!(
				"{green}✓{reset}",
				green = color::Fg(color::Green),
				reset = style::Reset
			)
		} else {
			format!(
				"{red}×{reset}",
				red = color::Fg(color::Red),
				reset = style::Reset
			)
		};

		println!(
			"  {enabled} {0} {faint}(https://haas.hackclub.com/apps/{0}){reset}",
			app.slug,
			enabled = enabled,
			faint = style::Faint,
			reset = style::Reset
		);
	}

	Ok(())
}
