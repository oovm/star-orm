use sea_orm::{
    prelude::DateTime, ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    SqlxPostgresConnector,
};

use game_orm::{leaderboard::Column, prelude::Leaderboard};
use Column::{EndTime, GameMode, GameTime};

async fn game_db() -> DatabaseConnection {
    SqlxPostgresConnector::connect(ConnectOptions::from("postgresql://localhost:5432/postgres")).await.unwrap()
}

#[tokio::test]
async fn select_daily() {
    let db = game_db().await;
    let cakes = Leaderboard::find()
        .filter(EndTime.gt(DateTime::from_timestamp_opt(0, 0)))
        .filter(GameMode.eq(1))
        .order_by_asc(GameTime)
        .limit(2)
        .all(&db)
        .await
        .unwrap();
    for cake in cakes {
        println!("{:?}", cake);
    }
}
