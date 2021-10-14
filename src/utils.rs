use std::error::Error;

use termion::style;

pub fn stringify_err(err: impl Error) -> String {
	format!(
		"{faint}Something went wrong:{reset} {}",
		err.to_string(),
		faint = style::Faint,
		reset = style::Reset
	)
}

pub fn stringify_status_err(err: reqwest::Error) -> String {
	if err.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
		format!(
			"🔒 Uh oh, you aren't logged in! Try {bold}haas auth login{reset}.",
			bold = style::Bold,
			reset = style::Reset
		)
	} else {
		format!(
			"{faint}Something went wrong:{reset} got HTTP status code {}",
			err.status()
				.unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
			faint = style::Faint,
			reset = style::Reset
		)
	}
}
