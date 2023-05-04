use crate::frontend::Error as FrontendError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("frontend")]
    Frontent(#[from] FrontendError),
}
