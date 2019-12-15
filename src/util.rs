use serde::de::{Deserialize, IntoDeserializer};

// Deserialize empty string as None.
// Used with `#[serde(deserialize_with = "empty_string_as_none")]`.
// Copied from https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some)
    }
}
