extern crate licder;
extern crate clap;

use clap::{Arg, App, SubCommand};
use licder::LicenseType;

fn main() {
    let matches = App::new("licder")
                            .version("0.1.0")
                            .author("Agoston Szepessy <agoston.the.dev@gmail.com>")
                            .about("CLI utility to download licenses")
                            .subcommand(SubCommand::with_name("fetch")
                                .about("Downloads specified licenses")
                                .arg(Arg::with_name("license")
                                     .multiple(true)
                                     .required(true)
                                )
                            ).subcommand(SubCommand::with_name("ls")
                                .about("Lists licenses available for download")
                            ).get_matches();

    match matches.subcommand() {
        ("fetch", Some(m)) => {
            for license in m.values_of("license").unwrap() {
                println!("{}", license);
            }
        }

        ("ls", _) => {
            for license in LicenseType::ls() {
                println!("{}", license);
            }
        }

        _ => {
            eprintln!("{}", matches.usage());
        }
    }
}
