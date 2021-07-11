use anyhow::Error;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{FromRow, SqlitePool};

use super::Crud;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Event {
    id: i32,
    name: String,
    description: String,
    latitude: f32,
    longitude: f32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    timestamp: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct EventForm {
    name: String,
    description: String,
    latitude: f32,
    longitude: f32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
}

#[async_trait]
impl Crud<EventForm> for Event {
    async fn create(db: &SqlitePool, form: &EventForm) -> Result<Self, Error> {
        let query = format!(
            "
INSERT INTO events ( name, description, latitude, longitude, start_time, end_time )
VALUES ( \"{}\", \"{}\", {}, {}, \"{}\", \"{}\" )
RETURNING *
        ",
            form.name,
            form.description,
            form.latitude,
            form.longitude,
            form.start_time,
            form.end_time
        );

        println!("{}", query);
        let event: Event = sqlx::query_as(
            query.as_str(),
        )
        .fetch_one(db)
        .await?;

        Ok(event)
    }
    async fn read(db: &SqlitePool, id: i32) -> Result<Self, Error> {
        // In the sql, `id as "id: _"` typechecks that field based on the struct, and not the database.
        let event: Event = sqlx::query_as!(
            Event,
            r#"
SELECT id as "id: _", name, description, latitude, longitude, start_time, end_time, timestamp
FROM events
WHERE id = ?1
        "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(event)
    }
    async fn update(db: &SqlitePool, id: i32, form: &EventForm) -> Result<Self, Error> {
        unimplemented!();
    }
    async fn delete(db: &SqlitePool, id: i32) -> Result<Self, Error> {
        unimplemented!();
    }
}

impl Event {
    pub async fn list(
        db: &SqlitePool,
        before_time: Option<NaiveDateTime>,
        after_time: Option<NaiveDateTime>,
        from_id: Option<i32>,
    ) -> Result<Vec<Event>, anyhow::Error> {
        // Can't typecheck a dynamic query. So here's a cool workaround:
        // (essentially if we don't want to constrain, we pass a value for which all columns will pass)
        // Sqlite is not a fan of chrono's MAXDATETIME or MINDATETIME, I feel that the year 3000 is sufficient futureproofing
        let before_uwrap = before_time.unwrap_or(NaiveDate::from_ymd(3000, 1, 1).and_hms(0, 0, 0)).to_string();
        let after_uwrap = after_time.unwrap_or(NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0)).to_string();
        let id_uwrap = from_id.unwrap_or(i32::MAX);

        println!("{}, {}", before_uwrap, after_uwrap);

        Ok(sqlx::query_as!(Event, r#"
            SELECT id as "id: _", name, description, latitude, longitude, start_time, end_time, timestamp
            FROM events
            WHERE start_time BETWEEN ?1 AND ?2
                AND id <= ?3
            LIMIT 50
                    "#, after_uwrap, before_uwrap, id_uwrap).fetch_all(db).await?)
    }
}
