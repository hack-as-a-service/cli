use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
	pub id: i32,
	pub slack_user_id: String,
	pub name: String,
	pub avatar: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Team {
	pub id: i32,
	pub name: Option<String>,
	pub slug: String,
	pub avatar: Option<String>,
	pub personal: bool,
}

#[derive(Deserialize, Debug)]
pub struct App {
	pub id: i32,
	pub slug: String,
	pub team_id: i32,
	pub enabled: bool,
}
