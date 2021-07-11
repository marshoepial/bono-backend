use anyhow::Error;
use sqlx::SqlitePool;
use warp::{
    reject::{not_found, Reject},
    Filter,
};

#[derive(Debug)]
pub struct InternalServerErrorRejection {
    pub err: Error,
}

impl Reject for InternalServerErrorRejection {}

pub fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn get_rejection_from_db(err: anyhow::Error) -> warp::Rejection {
    match err.downcast::<sqlx::Error>() {
        Ok(sqlx::Error::RowNotFound) => not_found(), //Error is a SQLError and row doesn't exist
        Ok(err) => warp::reject::custom(InternalServerErrorRejection {
            err: Error::new(err), //Error is a SQLError but we don't know how to handle it
        }),
        Err(err) => warp::reject::custom(InternalServerErrorRejection { err }), //Error is something else
    }
}
