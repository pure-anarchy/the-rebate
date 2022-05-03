use crate::crawler::fixture::stats::team::read_team_stats;
use crate::crawler::{create_client, request_page};
use select::predicate::Name;
use select::predicate::{Attr, Predicate};

#[tokio::test]
async fn team_stats_are_readable() {
    let client = create_client();

    let url = "https://fbref.com/en/matches/bf5f0f9e/Wolverhampton-Wanderers-Arsenal-February-10-2022-Premier-League";

    let match_page = request_page(&client, url).await.unwrap();

    let (h, a) = read_team_stats(&match_page);
    println!("{:#?}", &a);
}
