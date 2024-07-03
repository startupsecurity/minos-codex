use crate::config::Config;
use crate::regex::RegexCache;
use crate::secret_type::SecretType;
use crate::MinosCodexError;
use std::collections::HashMap;

pub struct Scanner {
    config: Config,
    regex_cache: RegexCache,
}

#[derive(Debug)]
pub struct FoundSecret {
    pub secret_type: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl Scanner {
    pub fn new(config: Config) -> Self {
        Scanner {
            config,
            regex_cache: RegexCache::new(),
        }
    }

    pub fn scan(&mut self, input: &str) -> Result<Vec<FoundSecret>, MinosCodexError> {
        let mut found_secrets = Vec::new();

        for secret_type in self.config.secret_types() {
            let regex = self.regex_cache.get_or_insert(&secret_type.regex)?;
            for capture in regex.captures_iter(input) {
                let matched = capture.get(0).unwrap();
                found_secrets.push(FoundSecret {
                    secret_type: secret_type.name.clone(),
                    value: matched.as_str().to_string(),
                    start: matched.start(),
                    end: matched.end(),
                });
            }
        }

        Ok(found_secrets)
    }
}
