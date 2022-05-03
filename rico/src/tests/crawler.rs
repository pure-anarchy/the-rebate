use crate::crawler::{
    create_client, fbref_local,
    fixture::{read_fixture_report, read_fixure_list},
    league::{crawl_teams, read_leagues, read_schedule_href},
    request_page,
    season::read_seasons,
};

#[tokio::test]
async fn whole_site_is_readable() {
    let client = create_client();
    let comps_page = request_page(&client, &fbref_local("/en/comps"))
        .await
        .unwrap();

    let leagues = read_leagues(&comps_page);

    for league in leagues.iter() {
        let seasons_page = request_page(&client, &fbref_local(&league.href))
            .await
            .unwrap();

        let seasons = read_seasons(&seasons_page);

        for season in seasons.iter() {
            let league_ovr = request_page(&client, &fbref_local(&season.href))
                .await
                .unwrap();

            crawl_teams(&league_ovr);

            let schedule_href = read_schedule_href(&league_ovr);

            let schedule_page =
                request_page(&client, &fbref_local(&schedule_href))
                    .await
                    .unwrap();

            let fixtures = read_fixure_list(&schedule_page);

            for fixture in fixtures {
                if let Some(report_href) = fixture.report_href {
                    read_fixture_report(
                        &request_page(&client, &fbref_local(&report_href))
                            .await
                            .unwrap(),
                    );
                }
            }
        }
    }
}
