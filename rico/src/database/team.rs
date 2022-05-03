use crate::error::Errable;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct TeamDb {
    team_id: i32,
    name: String,
    league_id: i16,
}

pub async fn insert_teams(
    db_conn: &PgPool,
    league_id: i16,
    team_names: impl Iterator<Item = &str>,
) -> Errable<()> {
    todo!()
}

pub async fn get_team_id(db_conn: &PgPool, name: &str) -> Errable<i32> {
    Ok(sqlx::query_scalar(&format!(
        "SELECT 
            team_id
         FROM
            tbl_team
         WHERE
            team_name = '{name}'
        "
    ))
    .fetch_one(db_conn)
    .await?)
}
