use std::{marker::PhantomData, time::Duration};

use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};
use sqlx::types::{
    chrono::{DateTime, Utc},
    JsonValue, Uuid,
};

use linq_core::QueryPlanner;

pub struct PostgresModel {}

/// ```sql
/// create table "Leaderboard"
/// (
///     game_id     uuid not null
///         constraint game_id primary key,
///     start_time  timestamp,
///     game_time   interval,
///     end_time    timestamp,
///     game_mode   integer,
///     game_result json,
///     game_mode2  pg_enum
/// );
/// ```
#[derive(Default)]
pub struct Leaderboard {
    row: PhantomData<LeaderboardRow>,
}

impl Leaderboard {
    pub fn batch(&self, count: usize) -> QueryPlanner<LeaderboardRow> {
        QueryPlanner::new(count)
    }
}

impl Leaderboard {
    pub fn select() -> String {
        let sql = "select * from Leaderboard";
        let dialect = PostgreSqlDialect {}; // or AnsiDialect
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        println!("AST: {:?}", ast);
        sql.to_string()
    }
}

#[rustfmt::skip]
pub struct LeaderboardRow {
    pub game_id     : Uuid,
    pub start_time  : Option<DateTime<Utc>>,
    pub game_time   : Option<Duration>,
    pub end_time    : Option<DateTime<Utc>>,
    pub game_mode   : Option<i32>,
    pub game_result : Option<JsonValue>,
    pub game_mode2  : Option<String>,
}

impl IntoIterator for Leaderboard {
    type Item = LeaderboardRow;
    type IntoIter = QueryPlanner<LeaderboardRow>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[test]
fn test() {
    let sql = include_str!("Leaderboard.sql");

    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let all = Leaderboard::default();
    #[linq]
    let q = all.batch(5).r#where(|row| row.game_mode == 1).order_by(|s| s.game_time);

    println!("AST: {:?}", ast);
}
