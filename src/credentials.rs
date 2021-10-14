use std::{collections::HashMap, fs};

use directories::ProjectDirs;
use termion::style;

use crate::utils::stringify_err;

lazy_static! {
	static ref DIRS: ProjectDirs = ProjectDirs::from("app.hackclub", "", "cli").unwrap();
}

pub fn set(key: String, value: &str) -> Result<(), String> {
	let config_dir = DIRS.config_dir();

	let mut config = get_all();

	config.insert(key, value.to_string());

	fs::create_dir_all(&config_dir).map_err(stringify_err)?;
	fs::write(
		config_dir.join("credentials.json"),
		serde_json::to_string(&config).map_err(stringify_err)?,
	)
	.map_err(stringify_err)?;

	Ok(())
}

pub fn get(key: &str) -> Option<String> {
	let config = get_all();

	config.get(key).map(|v| v.to_owned())
}

pub fn get_all() -> HashMap<String, String> {
	let path = DIRS.config_dir().join("credentials.json");
	let file = fs::read_to_string(path);

	match file {
		Ok(x) => {
			let parsed = serde_json::from_str::<HashMap<String, String>>(&x);

			match parsed {
				Ok(x) => x,
				Err(_) => HashMap::new(),
			}
		}
		Err(_) => HashMap::new(),
	}
}

pub fn get_token() -> Result<String, String> {
	get("token").ok_or(format!(
		"🔒 Uh oh, you aren't logged in! Try {bold}haas auth login{reset}.",
		bold = style::Bold,
		reset = style::Reset
	))
}
