use clap::Parser;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::Level;

#[derive(Debug)]
pub struct Settings {
    pub bind: SocketAddr,
    pub log_level: Level,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(short, long, default_value = "127.0.0.1:13005")]
    bind: SocketAddr,
    #[clap(short, long, default_value_t = Level::INFO)]
    log_level: Level,
}

impl Default for Settings {
    fn default() -> Self {
        let args = CliArgs::parse();
        Self {
            bind: args.bind.to_owned(),
            log_level: Level::from_str(args.log_level.as_str()).unwrap(),
        }
    }
}
