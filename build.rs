extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate regex;

use handlebars::Handlebars;

use std::fs::File;
use std::path::Path;
use std::env;

use serde_json::Value;

use regex::Regex;

fn main() {
    let handlebars = {
        let mut h = Handlebars::new();
        h.register_escape_fn(handlebars::no_escape);
        h
    };

    let mut json: Value = {
        let json = File::open(&"licenses.json").expect("Can't open JSON");
        serde_json::from_reader(json).expect("Can't parse JSON")
    };

    // License names contain dashes and periods and Rust enums can't have
    // variants that contain those characters, so they have to be removed
    let re = Regex::new(r"-|\.").unwrap();
    let parsed_json: serde_json::map::Map<_, _> = json.get_mut("licenses").unwrap().as_object_mut().unwrap().iter_mut().map(|(name, v)| {
        // Add abbreviation field because LicenseType::ls() should display both
        // abbreviated names and full names of licenses. The JSON only contains it
        // as a key
        v.as_object_mut().unwrap().insert("abbreviation".to_string(), json!(name));
        (re.replace_all(name, "").into_owned(), v.clone())
    }).collect();

    let formatted_json = json!({ "licenses": Value::Object(parsed_json) });

    // Generate file with licenses
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR doesn't exist");
    let output = Path::new(&out_dir).join("license_type.rs");
    let mut source_template = File::open(&"src/license_type.hbs.rs").expect("Can't find template to build licenses");
    let mut output_file = File::create(&output).expect("Can't create output file");

    if let Err(e) = handlebars.render_template_source_to_write(&mut source_template, &formatted_json, &mut output_file) {
        panic!("Failed to generate licenses! Error: {:?}", e);
    }
}
