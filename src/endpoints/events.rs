use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::SqlitePool;
use warp::Filter;

use crate::db::{
    events::{Event, EventForm},
    Crud,
};

use super::endpoint_utils::{get_rejection_from_db, with_db};

#[derive(Deserialize)]
pub struct GetEvents {
    pub before_time: Option<NaiveDateTime>,
    pub after_time: Option<NaiveDateTime>,
    pub from_id: Option<i32>,
}

impl GetEvents {
    pub fn get_events(
        db: SqlitePool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end()
            .and(warp::get())
            .and(with_db(db))
            .and(warp::query::<Self>())
            .and_then(Self::perform)
    }

    async fn perform(db: SqlitePool, options: Self) -> Result<impl warp::Reply, warp::Rejection> {
        match Event::list(&db, options.before_time, options.after_time, options.from_id).await {
            Ok(e) => Ok(warp::reply::json(&e)),
            Err(err) => Err(get_rejection_from_db(err)),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateEvent {
    #[serde(flatten)]
    pub form: EventForm,
}

impl CreateEvent {
    pub fn create_event(
        db: SqlitePool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("create")
            .and(warp::post())
            .and(with_db(db))
            .and(warp::body::content_length_limit(2048)) // 2kb limit
            .and(warp::body::json())
            .and_then(Self::perform)
    }

    async fn perform(db: SqlitePool, form: EventForm) -> Result<impl warp::Reply, warp::Rejection> {
        match Event::create(&db, &form).await {
            Ok(ev) => Ok(warp::reply::json(&ev)),
            Err(err) => Err(get_rejection_from_db(err)),
        }
    }
}
