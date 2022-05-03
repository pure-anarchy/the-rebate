use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

#[derive(Debug)]
pub struct Season {
    pub name: String,
    pub href: String,
}

/// get all seasons from 2017 and up, for any of europes top 5 leagues
pub fn read_seasons(seasons_page: &Document) -> Vec<Season> {
    seasons_page
        .find(
            Attr("id", "seasons")
                .descendant(Attr("data-stat", "season").descendant(Name("a"))),
        )
        .into_iter()
        .filter(|node| {
            node.text().split("-").collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap()
                > 2016
        })
        .map(|node| Season {
            name: node.text(),
            href: node.attr("href").unwrap().into(),
        })
        .collect()
}
