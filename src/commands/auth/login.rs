use std::{thread::sleep, time::Duration};

use clap::ArgMatches;
use reqwest::blocking as reqwest;
use serde::Deserialize;
use termion::style;

const CLIENT_ID: &str = "3939eb821756a14eb1f6745d33dfdd80";

#[derive(Deserialize)]
struct DeviceAuthorizationResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

pub fn login_command(_matches: &ArgMatches) -> Result<(), String> {
    let client = reqwest::Client::new();

    let resp = client
        .post("https://hackclub.app/api/oauth/device_authorization")
        .form(&[("client_id", CLIENT_ID)])
        .send()
        .map_err(|e| format!("Something went wrong: {}", e.to_string()))?
        .json::<DeviceAuthorizationResponse>()
        .map_err(|e| format!("Something went wrong: {}", e.to_string()))?;

    println!(
        "{faint}Copy your verification code:{reset} {} (🕐 expires in {} minutes)

{faint}Open {reset}{}{faint} in a browser to continue.{reset}

Waiting for authorization...",
        resp.user_code,
        chrono::Duration::seconds(resp.expires_in as i64).num_minutes(),
        resp.verification_uri,
        faint = style::Faint,
        reset = style::Reset,
    );

    let token = loop {
        sleep(Duration::from_secs(1));

        let resp = client
            .post("https://hackclub.app/api/oauth/token")
            .form(&[
                ("client_id", CLIENT_ID),
                ("device_code", &resp.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .map_err(|e| format!("Something went wrong: {}", e.to_string()))?
            .json::<AccessTokenResponse>()
            .map_err(|e| format!("Something went wrong: {}", e.to_string()))?;

        if let Some(token) = resp.access_token {
            break Ok(token);
        } else if let Some(error) = resp.error {
            if *"access_denied" == error {
                break Err("User denied access.".to_string());
            } else if *"expired_token" == error {
                break Err("Your verification code has expired.".to_string());
            } else if *"authorization_pending" == error {
                continue;
            } else {
                break Err(format!("Something unexpected happened: {}", error));
            }
        }
    }?;

    println!("\n✅ token: {}", token);

    Ok(())
}
