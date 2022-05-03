use sqlx::PgPool;

use crate::error::Errable;

#[derive(sqlx::FromRow)]
pub struct SeasonDb {
    pub season_id: i32,
    pub season_name: String,
    pub league_id: i16,
}

pub async fn insert_season(
    db_conn: &PgPool,
    season_name: &str,
    league_id: i16,
) -> Errable<SeasonDb> {
    Ok(sqlx::query_as(&format!(
        "
        INSERT INTO tbl_season
            (season_name, league_id)
        VALUES
            ('{season_name}', {league_id})
        ",
    ))
    .fetch_one(db_conn)
    .await?)
}

pub async fn all_seasons(
    db_conn: &PgPool,
    limit: u16,
) -> Errable<Vec<SeasonDb>> {
    Ok(sqlx::query_as(&format!(
        "SELECT
             season_name, season_id, league_id
         FROM
             season_tbl
         LIMIT {limit}
        "
    ))
    .fetch_all(db_conn)
    .await?)
}

pub async fn league_seasons(
    db_conn: &PgPool,
    league_id: i16,
) -> Errable<Vec<SeasonDb>> {
    todo!()
}

pub async fn get_season_by_id(
    db_conn: &PgPool,
    season_id: i16,
) -> Errable<SeasonDb> {
    Ok(sqlx::query_as(&format!(
        "SELECT
            season_id, season_name, league_id
         FROM
            season_tbl
         WHERE
            season_id = {season_id}
        "
    ))
    .fetch_one(db_conn)
    .await?)
}

/// because of the consistency in the naming of seasons, getting `2017-2018`
/// season, loads up all the seasons of all the leagues, you get a vec of
/// five seasons, in no particular order, each season belonging to a different
/// league
pub async fn get_season_by_name(
    db_conn: &PgPool,
    season_name: &str,
) -> Errable<Vec<SeasonDb>> {
    Ok(sqlx::query_as(&format!(
        "SELECT
            season_id, season_name, league_id
         FROM
            season_tbl
         WHERE
            season_name = {season_name}
        "
    ))
    .fetch_all(db_conn)
    .await?)
}
