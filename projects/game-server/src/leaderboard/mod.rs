use chrono::Utc;
use sea_orm::{
    prelude::DateTime, ActiveValue, ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, SqlxPostgresConnector,
};
use uuid::Uuid;
use ActiveValue::Set;

use game_orm::{
    log_result,
    log_result::Column::{EndTime, GameMode, GameTime},
    prelude::{Leaderboard, LogResult},
};

async fn game_db() -> DatabaseConnection {
    SqlxPostgresConnector::connect(ConnectOptions::from("postgresql://localhost:5432/postgres")).await.unwrap()
}

#[tokio::test]
async fn insert_record() {
    let db = game_db().await;
    let mut batch = vec![];
    for _ in 0..=10000 {
        let record = log_result::ActiveModel {
            game_id: Set(Uuid::new_v4()),
            is_valid: Set(true),
            start_time: Set(Utc::now().naive_utc()),
            game_time: Default::default(),
            end_time: Default::default(),
            ..Default::default() // no need to set primary key
        };
        batch.push(record);
    }
    LogResult::insert_many(batch).exec(&db).await.unwrap();
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
