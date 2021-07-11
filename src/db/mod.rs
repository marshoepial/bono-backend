use anyhow::Error;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub mod events;

#[async_trait]
pub trait Crud<T> {
    async fn create(db: &SqlitePool, form: &T) -> Result<Self, Error>
    where
        Self: Sized;
    async fn read(db: &SqlitePool, id: i32) -> Result<Self, Error>
    where
        Self: Sized;
    async fn update(db: &SqlitePool, id: i32, form: &T) -> Result<Self, Error>
    where
        Self: Sized;
    async fn delete(db: &SqlitePool, id: i32) -> Result<Self, Error>
    where
        Self: Sized;
}
