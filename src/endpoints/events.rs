use anyhow::Error;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::SqlitePool;
use warp::{Filter, reject::not_found};

use crate::db::{Crud, events::Event};

use super::endpoint_utils::{with_db, InternalServerErrorRejection};

#[derive(Deserialize)]
pub struct GetEvents {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

impl GetEvents{
    pub fn get_events(db: SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("events")
            .and(warp::get())
            .and(with_db(db))
            .and(warp::query::<Self>())
            .and_then(Self::perform)
    }

    async fn perform(db: SqlitePool, options: Self) -> Result<impl warp::Reply, warp::Rejection> {
        match Event::read(&db, 1).await {
            Ok(e) => Ok(warp::reply::json(&e)),
            Err(err) => {
                match err.downcast::<sqlx::Error>() {
                    Ok(sqlx::Error::RowNotFound) => Err(not_found()),   //Error is a SQLError and row doesn't exist
                    Ok(err) => Err(warp::reject::custom(InternalServerErrorRejection { err: Error::new(err) })),    //Error is a SQLError but we don't know how to handle it
                    Err(err) => Err(warp::reject::custom(InternalServerErrorRejection {err})),  //Error is something else
                }
            },
        }
    }
}