pub use self::{
    connect::connect_db,
    models::{leaderboard, prelude::*},
};

mod connect;
mod models;

pub use sea_orm::{entity::EntityTrait, DatabaseConnection};
