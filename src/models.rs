use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
	pub id: i32,
	pub slack_user_id: String,
	pub name: String,
	pub avatar: Option<String>,
}
