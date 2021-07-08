use anyhow::Error;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use super::Crud;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    description: String,
    latitude: f32,
    longitude: f32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    timestamp: NaiveDateTime
}

#[async_trait]
impl Crud<Event> for Event {
    async fn create(db: &SqlitePool, form: &Event) -> Result<Self, Error>{
        unimplemented!();
    }
    async fn read(db: &SqlitePool, id: i32) -> Result<Self, Error> {
        // In the sql, `id as "id: _"` typechecks that field based on the struct, and not the database.
        let event: Event = sqlx::query_as!(Event, r#"
SELECT id as "id: _", name, description, latitude, longitude, start_time, end_time, timestamp
FROM events
WHERE id = ?1
        "#, id)
            .fetch_one(db).await?;
        Ok(event)
    }
    async fn update(db: &SqlitePool, id: i32, form: &Event) -> Result<Self, Error> {
        unimplemented!();
    }
    async fn delete(db: &SqlitePool, id: i32) -> Result<Self, Error> {
        unimplemented!();
    }
}