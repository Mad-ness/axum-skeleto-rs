#![allow(dead_code)]
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct SharedDataContainer<T>(Arc<RwLock<T>>);

impl<T> From<T> for SharedDataContainer<T> {
    fn from(inner: T) -> SharedDataContainer<T> {
        Self(Arc::new(RwLock::new(inner)))
    }
}

impl<T> SharedDataContainer<T> {
    pub fn write(&self) -> impl std::future::Future<Output = RwLockWriteGuard<'_, T>> {
        self.0.write()
    }

    pub fn read(&self) -> impl std::future::Future<Output = RwLockReadGuard<'_, T>> {
        self.0.read()
    }
}
