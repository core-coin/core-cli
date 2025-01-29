use std::path::PathBuf;

use dirs::home_dir;
use structopt::StructOpt;
use types::DEFAULT_BACKEND;

#[derive(StructOpt, Debug)]
#[structopt(name = "core-cli")]
pub struct Cli {
    #[structopt(long, short, default_value = "go-core")]
    pub client: String,

    #[structopt(
        long,
        short,
        default_value = DEFAULT_BACKEND,
    )]
    pub backend: String,

    #[structopt(long, short)]
    pub datadir: Option<String>,
}

impl Cli {
    pub fn get_datadir(&self) -> PathBuf {
        match &self.datadir {
            Some(dir) => PathBuf::from(dir),
            None => {
                let mut default_path = home_dir().expect("Could not determine home directory");
                default_path.push(".core-cli/data");
                default_path
            }
        }
    }
}
