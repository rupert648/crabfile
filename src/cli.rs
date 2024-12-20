use clap::Parser;

const DEFAULT_HOME_PATH: &str = "~";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        value_parser,
        help = "Path to the configuration file",
        default_value = "./config.yaml"
    )]
    pub config: String,

    #[clap(
        long,
        value_parser,
        help = "Home directory path",
        default_value = DEFAULT_HOME_PATH
    )]
    pub home: String,
}
