use crate::database::team::insert_teams;
use crawler::{
    fbref_local,
    fixture::{read_fixture_report, read_fixure_list},
    league::{crawl_teams, read_leagues, read_schedule_href},
    request_page,
    season::read_seasons,
};
use database::{
    fixture::{insert_fixture, insert_fixture_report},
    league::insert_league,
    season::insert_season,
};
use error::Errable;
use reqwest::Client;
use sqlx::PgPool;

pub mod crawler;
pub mod database;
pub mod error;

#[cfg(test)]
mod tests;

pub async fn rico_rampage(client: &Client, db_conn: &PgPool) -> Errable<()> {
    let comps_page = request_page(client, &fbref_local("/en/comps")).await?;
    let leagues = read_leagues(&comps_page);

    for item in leagues.iter() {
        let league = insert_league(db_conn, &item.name).await?;
        let seasons_page =
            request_page(client, &fbref_local(&item.href)).await?;
        let seasons = read_seasons(&seasons_page);

        for val in seasons.iter() {
            let season =
                insert_season(db_conn, &val.name, league.league_id).await?;
            let overview_page =
                request_page(client, &fbref_local(&val.href)).await?;

            let teams = crawl_teams(&overview_page);
            let schedule_href = read_schedule_href(&overview_page);

            insert_teams(
                db_conn,
                league.league_id,
                teams.iter().map(|item| item.as_str()),
            )
            .await?;

            let schedule_page =
                request_page(client, &fbref_local(&schedule_href)).await?;
            let fixtures = read_fixure_list(&schedule_page);

            for game in fixtures.iter() {
                let fixture_report = if game.report_href.is_some() {
                    let report = read_fixture_report(
                        &request_page(
                            client,
                            &fbref_local(&game.report_href.clone().unwrap()),
                        )
                        .await?,
                    );
                    Some(report)
                } else {
                    None
                };

                let fixture =
                    insert_fixture(db_conn, season.season_id, game).await?;

                if let Some(report) = fixture_report {
                    insert_fixture_report(db_conn, fixture.fixture_id, &report)
                        .await?;
                }
            }
        }
    }

    Ok(())
}
