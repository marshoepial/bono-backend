use sqlx::SqlitePool;
use warp::Filter;

use crate::endpoints::events;

mod endpoints;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:bono.db").await?;

    let routes = events::GetEvents::get_events(pool)
        .or(warp::path("source").and(warp::get()).map(|| "https://github.com/marshoepial/bono-backend")); //agpl? idk, ianal
    warp::serve(routes).run(([127, 0, 0, 1], 8500)).await;

    Ok(())
}
