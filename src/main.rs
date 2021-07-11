use sqlx::SqlitePool;
use warp::Filter;

use crate::endpoints::events;

mod db;
mod endpoints;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:bono.db").await?;

    let routes = warp::path("events")
        .and(
            events::GetEvents::get_events(pool.clone())
                .or(events::CreateEvent::create_event(pool.clone())),
        )
        .or(warp::path("source")
            .and(warp::get())
            .map(|| "https://github.com/marshoepial/bono-backend")); //agpl? idk, ianal
    warp::serve(routes).run(([127, 0, 0, 1], 8500)).await;

    Ok(())
}
