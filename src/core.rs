use async_trait::async_trait;

///
/// Trait `Backend` is used for accessing to app state from frontend endpoints
///
#[async_trait]
pub trait Backend {
    fn counter(&mut self) -> usize;
}
