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

use crate::{config::ConfigFile, error::AppError};

mod config;
mod error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let git_commit = env!("GIT_COMMIT");
    let git_branch = env!("GIT_BRANCH");

    println!("{pkg_name} {pkg_version}-{git_commit}({git_branch})");

    // TODO: Allow specifying the configuration file path via command-line arguments or environment variables.
    let config = ConfigFile::load_or_create_if_missing("config.json")?;

    println!("Loaded configuration with {} entries", config.entries.len());
    for entry in config.entries {
        // TODO: Remove after testing, to prevent leaking sensitive information (e.g., API keys, secrets).
        println!("Entry: {:?}", entry);
    }

    Ok(())
}
