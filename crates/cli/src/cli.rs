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

    #[structopt(long, short, default_value = "./data")]
    pub datadir: String,
}
