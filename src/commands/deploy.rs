use clap::ArgMatches;

pub fn deploy_command(matches: &ArgMatches) {
    let app = matches.value_of("app").unwrap();

    // ToDO: actually deploy
    println!("Deploying {}...", app);
}
