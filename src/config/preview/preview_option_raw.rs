use bytesize::ByteSize;
use serde::{de::Unexpected, Deserialize, Deserializer};
use toml::Value;

use crate::types::option::preview::{PreviewProtocol, XDGThumbSizes};

use crate::utils::serde::{default_max_preview_size, default_true};

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewOptionRaw {
    #[serde(
        default = "default_max_preview_size",
        deserialize_with = "deserialize_max_preview_size"
    )]
    pub max_preview_size: u64,
    #[serde(default)]
    pub preview_protocol: PreviewProtocol,
    #[serde(default)]
    pub preview_script: Option<String>,
    #[serde(default = "default_true")]
    pub use_xdg_thumbs: bool,
    #[serde(default)]
    pub xdg_thumb_size: XDGThumbSizes,
    #[serde(default)]
    pub preview_shown_hook_script: Option<String>,
    #[serde(default)]
    pub preview_removed_hook_script: Option<String>,
}

// This should be deleted maybe. I don't see where it is invoked.
impl std::default::Default for PreviewOptionRaw {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_protocol: PreviewProtocol::Disabled,
            preview_script: None,
            use_xdg_thumbs: true,
            xdg_thumb_size: XDGThumbSizes::XLarge,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}

fn deserialize_max_preview_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    let string = match value {
        Value::String(s) => s,
        Value::Integer(i) => (i as u64).to_string(),
        v => {
            return Err(serde::de::Error::invalid_type(
                Unexpected::Other(v.type_str()),
                &"String or Integer",
            ))
        }
    };

    let size = string
        .parse::<ByteSize>()
        .map_err(serde::de::Error::custom)?;

    Ok(size.as_u64())
}
