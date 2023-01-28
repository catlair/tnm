use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("tnm")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Pacman Development Team")
        //
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("list registries")
        )
        //
        .subcommand(
            Command::new("use")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("registry")
                        .help("registry")
                        .required(true)
                        .action(ArgAction::Set)
                ),
        )
        .subcommand(
            Command::new("test")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("registry")
                        .help("registry")
                        .action(ArgAction::Set)
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("name")
                        .help("name")
                        .required(true)
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("registry")
                        .help("registry")
                        .required(true)
                        .action(ArgAction::Set)
                ) .arg(
                    Arg::new("home")
                        .help("home")
                        .action(ArgAction::Set)
                )
                ,
        )  .subcommand(
            Command::new("del")
                .alias("rm")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("registry")
                        .help("registry")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {}...", values);
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if sync_matches.get_flag("info") {
                println!("Retrieving info for {}...", values);
            } else {
                println!("Installing {}...", values);
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if let Some(queries) = query_matches.get_many::<String>("search") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {}...", comma_sep);
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
