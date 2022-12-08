use sea_orm::{ConnectOptions, DbErr, SqlxPostgresPoolConnection};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let out = SqlxPostgresPoolConnection::connect(ConnectOptions::new("postgresql://localhost:5432/postgres".to_string()))?;
    Ok(out)
}
