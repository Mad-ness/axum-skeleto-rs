use crate::{core::Backend, utils::SharedDataContainer};
use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;
use thiserror::Error as ThisError;

pub async fn run_api<B>(bind: &SocketAddr, backend: SharedDataContainer<B>) -> Result<(), Error>
where
    B: Backend + Send + Sync + Clone + 'static,
{
    let app = Router::new().route("/", get(counting)).with_state(backend);

    axum::Server::bind(bind)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn counting<B>(State(backend): State<SharedDataContainer<B>>) -> String
where
    B: Backend + Send + Sync + Clone + 'static,
{
    format!("{}", backend.write().await.counter())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("stdio")]
    Std(#[from] std::io::Error),
    #[error("hyper")]
    Hyper(#[from] hyper::Error),
}
