use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

#[derive(Debug)]
pub struct League {
    pub name: String,
    pub href: String,
}

/// get a list of europes, top 5 Leagues
/// epl, laliga, serie-a, bundesliga, ligue 1
pub fn read_leagues(comps_page: &Document) -> Vec<League> {
    let leagues = comps_page
        .find(
            Attr("id", "comps_club")
                .descendant(Attr("data-stat", "league_name")),
        )
        .into_iter()
        .skip(1)
        .take(5)
        .map(|node| League {
            name: node.first_child().unwrap().text(),
            href: node.first_child().unwrap().attr("href").unwrap().into(),
        })
        .collect::<Vec<League>>();

    assert_eq!(leagues.len(), 5);
    leagues
}

pub fn crawl_teams(page: &Document) -> Vec<String> {
    page.find(Name("table").descendant(Name("tbody")))
        .next()
        .unwrap()
        .find(Attr("data-stat", "squad"))
        .into_iter()
        .map(|node| node.text().trim().into())
        .collect::<Vec<String>>()
}

pub fn read_schedule_href(page: &Document) -> String {
    page.find(Name("a"))
        .into_iter()
        .filter(|node| node.text().trim() == "Scores & Fixtures")
        .take(1)
        .map(|node| node.attr("href").unwrap().to_owned())
        .collect::<String>()
}
