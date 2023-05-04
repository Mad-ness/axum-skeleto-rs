mod core;
mod error;
mod frontend;
mod settings;
mod utils;

use crate::{core::Backend, frontend::run_api, settings::Settings, utils::SharedDataContainer};
use std::net::SocketAddr;

// An example which implements Backend trait
#[derive(Default, Clone)]
pub struct Counter(usize);

impl Backend for Counter {
    fn counter(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

fn setup_logging() {
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() {
    setup_logging();
    let _settings = Settings::default();
    let counter = SharedDataContainer::from(Counter::default());
    let bind: SocketAddr = "127.0.0.1:3305".parse().unwrap();
    let _ = run_api(&bind, counter).await;
}
