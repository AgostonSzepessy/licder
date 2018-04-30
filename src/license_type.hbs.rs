use std::convert::From;

use self::LicenseType::*;

pub enum LicenseType {
    {{#each licenses}}
        {{~@key}},
    {{/each}}
    None,
}

impl LicenseType {
    /// Returns the name of the license.
    pub fn name(&self) -> &'static str {
        match *self {
            {{~#each licenses}}
                {{@key}} => "{{~name}}",
            {{/each}}
                None => unreachable!()
        }
    }

    /// Returns the URL at which the license is located.
    pub fn url(&self) -> &'static str {
        match *self {
            {{~#each licenses}}
                {{@key}} => "{{~url}}",
            {{~/each}}
                None => unreachable!()
        }
    }
}

impl From<String> for LicenseType {
    fn from(s: String) -> Self {
        LicenseType::from(&*s)
    }
}

impl<'a> From<&'a str> for LicenseType {
    fn from(s: &'a str) -> Self {
        match s {
            {{~#each licenses}}
                "{{~@key}}" => LicenseType::{{~@key}},
            {{~/each}}
                _ => LicenseType::None,
        }
    }
}
