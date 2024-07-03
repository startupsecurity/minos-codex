use crate::secret_type::SecretType;
use crate::MinosCodexError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct SecretTypeWrapper {
    secret_type: SecretType,
}

pub struct Config {
    secret_types: HashMap<String, SecretType>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            secret_types: HashMap::new(),
        }
    }

    pub fn load_from_directory<P: AsRef<Path>>(dir_path: P) -> Result<Self, MinosCodexError> {
        let mut config = Config::new();
        let dir = fs::read_dir(dir_path)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                let secret_type = Config::load_secret_type_from_file(&path)?;
                config
                    .secret_types
                    .insert(secret_type.name.clone(), secret_type);
            }
        }

        Ok(config)
    }

    fn load_secret_type_from_file(file_path: &PathBuf) -> Result<SecretType, MinosCodexError> {
        let contents = fs::read_to_string(file_path)?;
        let wrapper: SecretTypeWrapper = toml::from_str(&contents)?;
        wrapper
            .secret_type
            .validate()
            .map_err(|e| MinosCodexError::ValidationError(e))?;
        Ok(wrapper.secret_type)
    }

    pub fn get_secret_type(&self, name: &str) -> Option<&SecretType> {
        self.secret_types.get(name)
    }

    pub fn secret_types(&self) -> impl Iterator<Item = &SecretType> {
        self.secret_types.values()
    }
}
