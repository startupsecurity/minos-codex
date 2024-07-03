use crate::MinosCodexError;
use regex::Regex;
use std::collections::HashMap;

pub struct RegexCache {
    cache: HashMap<String, Regex>,
}

impl RegexCache {
    pub fn new() -> Self {
        RegexCache {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, pattern: &str) -> Result<&Regex, MinosCodexError> {
        if !self.cache.contains_key(pattern) {
            let regex = Regex::new(pattern)?;
            self.cache.insert(pattern.to_string(), regex);
        }
        Ok(self.cache.get(pattern).unwrap())
    }
}
