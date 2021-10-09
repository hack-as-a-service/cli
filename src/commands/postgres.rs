use std::process::{self, Command};

use clap::ArgMatches;
use termion::{color, style};

pub fn postgres_command(matches: &ArgMatches) -> Result<(), String> {
    let app = matches.value_of("app").unwrap();
    println!("Attaching to app {}...\n", app);

    let status = Command::new("psql")
        .arg("-v")
        .arg(format!(
            "PROMPT1=📡 [ {}{}{}{}{}@%M{} ] ==> ",
            color::Fg(color::Blue),
            style::Bold,
            app,
            style::Reset,
            style::Faint,
            style::Reset
        ))
        // TODO: actually fetch the correct URL
        .arg("postgres://haas@localhost:5432/haas")
        .status()
        .map_err(|_| String::from("Error running psql"))?;

    process::exit(status.code().unwrap_or(0))
}
