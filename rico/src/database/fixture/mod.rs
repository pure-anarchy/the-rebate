use sqlx::PgPool;

use crate::{
    crawler::fixture::{
        stats::goalkeeper::GkStats,
        stats::{shots::ShotStats, PlayerStats},
        Fixture, FixtureReport,
    },
    error::Errable,
};

use super::team::get_team_id;

#[derive(sqlx::FromRow)]
pub struct FixtureDb {
    pub fixture_id: i32,
    pub season_id: i32,
    pub matchweek: i16,
    pub home_team_id: i32,
    pub away_team_id: i32,
    pub fixture_date: Option<chrono::NaiveDate>,
    pub fixture_time: Option<chrono::NaiveTime>,
    pub attendance: Option<i32>,
}

pub async fn insert_fixture(
    db_conn: &PgPool,
    season_id: i32,
    fixture: &Fixture,
) -> Errable<FixtureDb> {
    let home_team_id = get_team_id(db_conn, &fixture.home_team).await?;
    let away_team_id = get_team_id(db_conn, &fixture.away_team).await?;

    Ok(sqlx::query_as!(
        FixtureDb,
        r#"INSERT INTO fixture_tbl
            (season_id, home_team_id, away_team_id, fixture_date, fixture_time)
         VALUES
            ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        season_id,
        home_team_id,
        away_team_id,
        fixture.date,
        fixture.time
    )
    .fetch_one(db_conn)
    .await?)
}

pub async fn insert_fixture_report(
    db_conn: &PgPool,
    fixture_id: i32,
    report: &FixtureReport,
) -> Errable<()> {
    sqlx::query(
        r#"INSERT INTO fixture_report_tbl
            ( fixture_id, home_goals, away_goals, home_xg, away_xg, home_match_record,
              away_match_record, home_passes, away_passes, home_accurate_passes, 
              away_accurate_passes, home_shots, away_shots, home_shots_on_target,
              away_shots_on_target, home_gk_saves, away_gk_saves, home_yellow_cards,
              away_yellow_cards, home_fouls, away_fouls, home_corners, away_corners,
              home_tackles, away_tackles, home_interceptions, away_interceptions,
              home_aerials_won, away_aerials_won, home_offsides, away_offsides,
              home_goal_kicks, away_goal_kicks, home_throw_ins, away_throw_ins,
              home_long_balls, away_long_balls
            )
        VALUES
            (  $1, $2, $3, $4, $5, $6,
               $7, $8, $9, $10,
               $11, $12, $13, $14,
               $15, $16, $17, $18,
               $19, $20, $21, $22, $23,
               $24, $25, $26, $27,
               $28, $29, $30, $31,
               $32, $33, $34, $35,
               $36, $37
            )
        "#,
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(())
}

pub async fn insert_player_stats(db_conn: &PgPool, stats: Vec<PlayerStats>) {}

pub async fn insert_gk_stats(db_conn: &PgPool, stats: Vec<GkStats>) {}

pub async fn insert_shot_stats(db_conn: &PgPool, stats: Vec<ShotStats>) {}
