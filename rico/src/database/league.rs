use sqlx::PgPool;

use crate::error::Errable;

#[derive(sqlx::FromRow)]
pub struct LeagueDb {
    pub league_id: i16,
    pub league_name: String,
}

pub async fn insert_league(
    db_conn: &PgPool,
    league_name: &str,
) -> Errable<LeagueDb> {
    Ok(sqlx::query_as(&format!(
        "INSERT INTO tbl_league
            (league_name)
         VALUES
            ('{league_name}')
        "
    ))
    .fetch_one(db_conn)
    .await?)
}

pub async fn all_leagues(db_conn: &PgPool) -> Errable<Vec<LeagueDb>> {
    Ok(sqlx::query_as(&format!(
        "SELECT
            league_name, league_id
         FROM
            tbl_league
        "
    ))
    .fetch_all(db_conn)
    .await?)
}

pub async fn get_league_by_id(
    db_conn: &PgPool,
    league_id: i16,
) -> Errable<LeagueDb> {
    Ok(sqlx::query_as(&format!(
        "SELECT
            league_id, league_name,
         FROM
            tbl_league
         WHERE
            league_name = {league_id}
        "
    ))
    .fetch_one(db_conn)
    .await?)
}

/// do a text search for the using the name of the league and return all
/// the items that match the the text search, ordered by how much of a match
/// the item is to the search query
pub async fn search_league(
    db_conn: &PgPool,
    search: &str,
) -> Errable<Vec<LeagueDb>> {
    Ok(sqlx::query_as(&format!(
        "SELECT
            league_name, league_id
         FROM
            tbl_league
         WHERE
        "
    ))
    .fetch_all(db_conn)
    .await?)
}
