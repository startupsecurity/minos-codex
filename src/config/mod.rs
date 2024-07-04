use crate::secret_type::SecretType;
use crate::MinosCodexError;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::Utf8Error;

impl From<Utf8Error> for MinosCodexError {
    fn from(err: Utf8Error) -> Self {
        MinosCodexError::ConfigLoadError(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}

#[derive(Deserialize)]
struct SecretTypeWrapper {
    secret_type: SecretType,
}

#[derive(RustEmbed)]
#[folder = "detections/"]
struct ConfigAssets;

pub struct Config {
    secret_types: HashMap<String, SecretType>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            secret_types: HashMap::new(),
        }
    }

    pub fn load_from_embedded() -> Result<Self, MinosCodexError> {
        let mut config = Config::new();

        for file in ConfigAssets::iter() {
            if file.ends_with(".toml") {
                let file_data = ConfigAssets::get(&file).ok_or_else(|| {
                    MinosCodexError::ConfigLoadError(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "File not found",
                    ))
                })?;
                let contents = std::str::from_utf8(file_data.data.as_ref())?;
                let secret_type = Config::load_secret_type_from_data(contents)?;
                config
                    .secret_types
                    .insert(secret_type.name.clone(), secret_type);
            }
        }

        Ok(config)
    }

    fn load_secret_type_from_data(data: &str) -> Result<SecretType, MinosCodexError> {
        let wrapper: SecretTypeWrapper = toml::from_str(data)?;
        wrapper
            .secret_type
            .validate()
            .map_err(MinosCodexError::ValidationError)?;
        Ok(wrapper.secret_type)
    }

    pub fn get_secret_type(&self, name: &str) -> Option<&SecretType> {
        self.secret_types.get(name)
    }

    pub fn secret_types(&self) -> impl Iterator<Item = &SecretType> {
        self.secret_types.values()
    }
}
