use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretType {
    pub name: String,
    pub regex: String,
    pub description: Option<String>,
    pub examples: HashSet<String>,
    #[serde(default)]
    pub false_positives: HashSet<String>,
}

impl SecretType {
    pub fn new(
        name: String,
        regex: String,
        description: Option<String>,
        examples: HashSet<String>,
        false_positives: HashSet<String>,
    ) -> Self {
        SecretType {
            name,
            regex,
            description,
            examples,
            false_positives,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // 1. Check if the regex is valid
        let re = match regex::Regex::new(&self.regex) {
            Ok(re) => re,
            Err(e) => return Err(format!("Invalid regex: {}", e)),
        };

        // 2. Ensure that all examples match the regex
        for example in &self.examples {
            if !re.is_match(example) {
                return Err(format!("Example '{}' does not match the regex", example));
            }
        }

        // 3. Ensure that all false positives do not match the regex
        for false_positive in &self.false_positives {
            if re.is_match(false_positive) {
                return Err(format!(
                    "False positive '{}' matches the regex",
                    false_positive
                ));
            }
        }

        Ok(())
    }
}
