use clap::ArgMatches;

pub fn deploy_command(matches: &ArgMatches) -> Result<(), String> {
    let app = matches.value_of("app").unwrap();

    // ToDO: actually deploy
    println!("Deploying {}...", app);

    Ok(())
}
