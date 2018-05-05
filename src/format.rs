//! Utility module to convert data to types and back by
//! specific formats like: JSON, BSON, TOML, YAML, XML.
//!
//! All types here are lazy and it's necessary to
//! use `Into` and `From` traits to get (convert) the data.

use failure::{Error, err_msg};
use serde::{Deserialize, Serialize};
use serde_json;

/// A representation of a value which can be stored and restored as a text.
pub type Text = Result<String, Error>;

/// A representation of a value which can be stored and restored as a binary.
pub type Binary = Result<Vec<u8>, Error>;

/// A representation of an empty data. Nothing stored. Nothing restored.
pub struct Nothing;

impl Into<Text> for Nothing {
    fn into(self) -> Text {
        Err(err_msg("nothing"))
    }
}

impl From<Text> for Nothing {
    fn from(_: Text) -> Nothing {
        Nothing
    }
}

impl Into<Binary> for Nothing {
    fn into(self) -> Binary {
        Err(err_msg("nothing"))
    }
}

impl From<Binary> for Nothing {
    fn from(_: Binary) -> Nothing {
        Nothing
    }
}

/// A representation of a JSON data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Json
/// let dump = Json(&data);
///
/// // Converts JSON string to a data (lazy).
/// let Json(data) = dump;
/// ```
pub struct Json<T>(pub T);

impl<'a, T> Into<Text> for Json<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Text {
        serde_json::to_string(&self.0).map_err(Error::from)
    }
}

impl<T> From<Text> for Json<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Text) -> Self {
        match value {
            Ok(data) => Json(serde_json::from_str(&data).map_err(Error::from)),
            Err(reason) => Json(Err(reason)),
        }
    }
}

impl<'a, T> Into<Binary> for Json<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Binary {
        serde_json::to_vec(&self.0).map_err(Error::from)
    }
}

impl<T> From<Binary> for Json<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Binary) -> Self {
        match value {
            Ok(data) => Json(serde_json::from_slice(&data).map_err(Error::from)),
            Err(reason) => Json(Err(reason)),
        }
    }
}
