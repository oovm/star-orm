use sea_orm::{prelude::DateTime, ColumnTrait, EntityTrait, QueryOrder, QuerySelect};

#[tokio::test]
async fn select() {
    let db = game_db().await.unwrap();
    let cakes = game_orm::leaderboard::Entity::find()
        .filter(game_orm::leaderboard::Column::EndTime.gt(DateTime::from_timestamp_opt(0, 0)))
        .filter(game_orm::leaderboard::Column::GameMode.eq(1))
        .order_by_asc(game_orm::leaderboard::Column::GameTime)
        .limit(2)
        .all(&db)
        .await
        .unwrap();
    for cake in cakes {
        println!("{:?}", cake);
    }
}
