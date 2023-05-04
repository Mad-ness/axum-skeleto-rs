use async_trait::async_trait;

#[async_trait]
pub trait Backend {
    fn counter(&mut self) -> usize;
}
