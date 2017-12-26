//! Fractal Global Location module
//!
//! This module holds the Fractal Global Address, and Geological location data type objects.

// #[cfg(feature = "json-types")]
// use rustc_serialize::json;

/// The particulars of the place where an organization or person resides
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]//RustcEncodable, RustcDecodable)]
pub struct Address {
    /// First Address
    address1: String,
    /// Second Address
    address2: Option<String>,
    /// The City
    city: String,
    /// The State
    state: String,
    /// The Zip Code
    zip: String,
    /// The Country
    country: String,
}

impl Address {
    /// Creates a new `Address`
    pub fn new<S: AsRef<str>>(address1: S,
                              address2: Option<S>,
                              city: S,
                              state: S,
                              zip: S,
                              country: S)
                              -> Address {
        Address {
            address1: address1.as_ref().to_owned(),
            address2: match address2 {
                Some(s) => Some(s.as_ref().to_owned()),
                None => None,
            },
            city: city.as_ref().to_owned(),
            state: state.as_ref().to_owned(),
            zip: zip.as_ref().to_owned(),
            country: country.as_ref().to_owned(),
        }
    }

    /// Returns address line 1
    pub fn get_address1(&self) -> &str {
        &self.address1
    }

    /// Returns address line 2
    pub fn get_address2(&self) -> Option<&str> {
        match self.address2 {
            Some(ref addr2) => Some(addr2),
            None => None,
        }
    }

    /// Returns the city
    pub fn get_city(&self) -> &str {
        &self.city
    }

    /// Returns the state
    pub fn get_state(&self) -> &str {
        &self.state
    }

    /// Returns the zip code
    pub fn get_zip(&self) -> &str {
        &self.zip
    }

    /// Returns the country
    pub fn get_country(&self) -> &str {
        &self.country
    }
}
