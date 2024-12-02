use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "core-cli")]
pub struct Cli {
    #[structopt(long, default_value = "go-core")]
    pub client: String,

    #[structopt(long, default_value = "http://localhost:8545")]
    pub backend: String,
}
