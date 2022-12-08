use game_orm::{connect_db, EntityTrait, Leaderboard};

#[tokio::test]
async fn select() {
    let db = connect_db().await.unwrap();

    let cakes = Leaderboard::find().all(&db).await.unwrap();

    for cake in cakes {
        println!("{:?}", cake);
    }
}
