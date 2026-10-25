// Copyright 2026 ScaryBandana
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub entries: Vec<DdnsEntryConfig>,
}

// Placeholder for provider-specific DDNS entry configurations (e.g., DynDNS2, IONOS).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "provider")]
pub enum DdnsEntryConfig {}

pub struct ConfigFile {}

impl ConfigFile {
    pub fn load_or_create_if_missing(path: impl Into<PathBuf>) -> Result<AppConfig, AppError> {
        let path = path.into();

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                let config = serde_json::from_str(&content)?;
                Ok(config)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Self::write_default_config(&path)?;

                println!(
                    "Configuration file '{}' not found. Created default configuration file.",
                    path.display()
                );

                Ok(AppConfig::default())
            }
            Err(error) => Err(error.into()),
        }
    }

    fn write_default_config(path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&AppConfig::default())?;
        std::fs::write(path, content)?;

        Ok(())
    }
}
