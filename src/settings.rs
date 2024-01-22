use serde::{Deserialize, Serialize};

// Describe the settings your policy expects when
// loaded by the policy server.
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() {
        let settings = Settings {};
        assert!(settings.validate().is_ok());
    }
}
