use anyhow::Error;
use sqlx::SqlitePool;
use warp::{Filter, reject::Reject};

#[derive(Debug)]
pub struct InternalServerErrorRejection {
    pub err: Error,
}

impl Reject for InternalServerErrorRejection {}


pub fn with_db(db: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}