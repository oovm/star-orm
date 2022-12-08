create table "Leaderboard"
(
    game_id     uuid not null
        constraint game_id primary key,
    start_time  timestamp,
    game_time   interval,
    end_time    timestamp,
    game_mode   integer,
    game_result json,
    game_mode2  pg_enum
);
