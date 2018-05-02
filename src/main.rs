extern crate licder;
extern crate clap;
extern crate reqwest;

use clap::{Arg, App, SubCommand};
use reqwest::Response;
use licder::LicenseType;

use std::fs::File;
use std::io::Write;

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
            let licenses = m.values_of("license").unwrap();

            // If only 1 license is specified, the file is saved as "LICENSE"
            if licenses.len() == 1 {
                if let Some(license) = create_license(licenses.last().unwrap()) {
                    if let Some(mut res) = download(license.url(), license.abbreviation()) {
                        save(&mut res, "");
                    }
                }
            } else {
                // If more than 1 license is specified licenses are saved as
                // "LICENSE-license-abbreviation"
                let mut responses = Vec::new();
                
                // Download all licenses
                for name in m.values_of("license").unwrap() {
                    println!("{}", &name);

                    if let Some(license) = create_license(name) {
                        if let Some(res) = download(license.url(), license.abbreviation()) {
                            responses.push((res, license.abbreviation()));
                        }
                    }
                }

                // Write licenses to disk
                for (mut res, license_abbrev) in responses {
                    save(&mut res, license_abbrev);
                }
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

fn create_license(name: &str) -> Option<LicenseType> {
    match LicenseType::try_from_str(&name) {
        Ok(l) => Some(l),
        Err(e) => {
            eprintln!("Error trying to parse license {}: {}", name, e);
            None
        }
    }
}

fn download(url: &str, name: &str) -> Option<Response> {
    match reqwest::get(url) {
        Ok(res) => Some(res),
        Err(e) => {
            eprintln!("Unable to download license {}: {}", name, e);
            None
        }
    }
}

fn save(res: &mut Response, license_abbrev: &str) {
    let filename = if license_abbrev.len() != 0 {
        format!("LICENSE-{}", license_abbrev)
    } else {
        "LICENSE".to_string()
    };

    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to save {}: {}", filename, e);
            return;
        }
    };

    match res.copy_to(&mut file) {
        Ok(_) => {
            match file.flush() {
                Ok(_) => println!("{} saved", filename),
                Err(e) => {
                    eprintln!("Unable to save {}: {}", filename, e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Unable to write {} to disk: {}", filename, e);
            return;
        }
    }

}
