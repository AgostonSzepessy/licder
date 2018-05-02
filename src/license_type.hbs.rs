use std::fmt;
use std::error;

use regex::Regex;

use self::LicenseType::*;

/// The types of errors that can occur from a `LicenseType`.
#[derive(Debug, PartialEq)]
pub enum LicenseTypeError {
    /// License not supported by licder
    LicenseNotSupported(String),
}

impl fmt::Display for LicenseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LicenseTypeError::LicenseNotSupported(ref err) => write!(f, "License {} not supported by licder", err),
        }
    }
}

impl error::Error for LicenseTypeError {
    fn description(&self) -> &str {
        match *self {
            LicenseTypeError::LicenseNotSupported(..) => "License not supported by licder",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LicenseTypeError::LicenseNotSupported(..) => None,
        }
    }
}

/// The different types of licenses that licder can download.
#[derive(Debug, PartialEq)]
pub enum LicenseType {
    {{#each licenses}}
        {{~@key}},
    {{/each}}
}

impl LicenseType {

    /// Tries to create an instance of `LicenseType` from the abbreviation of the license.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate licder;
    ///
    /// let license = licder::LicenseType::try_from_str("MPL-2.0");
    /// assert_eq!(Ok(licder::LicenseType::MPL20), license);
    /// ```
    pub fn try_from_str(s: &str) -> Result<Self, LicenseTypeError> {
        let re = Regex::new(r"-|\.").unwrap();
        let res = re.replace_all(s, "");
        match &*res {
            {{~#each licenses}}
                "{{~@key}}" => Ok(LicenseType::{{~@key}}),
            {{~/each}}
                _ => Err(LicenseTypeError::LicenseNotSupported(s.to_string())),
        }
    }

    pub fn try_from_string(s: String) -> Result<Self, LicenseTypeError> {
        Self::try_from_str(&*s)
    }

    /// Returns the name of the license.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate licder;
    ///
    /// let license = licder::LicenseType::try_from_str("MPL-2.0").unwrap();
    /// assert_eq!("Mozilla Public License 2.0", license.name());
    /// ```
    pub fn name(&self) -> &'static str {
        match *self {
            {{~#each licenses}}
                {{@key}} => "{{~name}}",
            {{/each}}
        }
    }

    /// Returns the URL at which the license is located.
    ///
    /// # Example
    /// 
    /// ```
    /// extern crate licder;
    ///
    /// let license = licder::LicenseType::try_from_str("MPL-2.0").unwrap();
    /// assert_eq!("https://www.mozilla.org/media/MPL/2.0/index.815ca599c9df.txt", license.url());
    /// ```
    pub fn url(&self) -> &'static str {
        match *self {
            {{~#each licenses}}
                {{@key}} => "{{~url}}",
            {{~/each}}
        }
    }

    /// Returns the abbreviation of the license.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate licder;
    ///
    /// let license = licder::LicenseType::try_from_str("MPL-2.0").unwrap();
    /// assert_eq!("MPL-2.0", license.abbreviation());
    /// ```
    pub fn abbreviation(&self) -> &'static str {
        match *self {
            {{~#each licenses}}
                {{@key}} => "{{~abbreviation}}",
            {{~/each}}
        }
    }

    /// Returns a `Vec` containing all licenses with their abbreviation and full name
    /// separated by a dash. E.g. `MPL 2.0 - Mozilla Public License 2.0`
    pub fn ls() -> Vec<&'static str> {
        vec![
            {{~#each licenses}}
                "{{abbreviation}} - {{name}}",
            {{/each}}
        ]
    }
}
