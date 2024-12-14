use std::{env, fs, path::PathBuf};
use uuid::Uuid;

use cli_error::CliError;

pub fn create_tmp_dir(mut location: Option<PathBuf>) -> PathBuf {
    if location.is_none() {
        location = Some(env::temp_dir());
    }

    let dir = location
        .unwrap()
        .join("core-cli-test")
        .join(Uuid::new_v4().to_string());

    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn remove_tmp_dir(dir: PathBuf) -> Result<(), CliError> {
    fs::remove_dir_all(dir)?;
    Ok(())
}
