use crate::crawler::fixture::stats::{
    goalkeeper::read_gk_stats, shots::read_shot_stats, team::read_team_stats,
};
use std::str::FromStr;

use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Name, Predicate},
};

use crate::crawler::fixture::stats::{
    goalkeeper::GkStats, shots::ShotStats, PlayerStats,
};

use self::stats::{read_player_stats, team::TeamStats};

use super::data_stat;

pub mod stats;

#[derive(Debug)]
pub struct Fixture {
    pub match_week: i16,
    pub home_team: String,
    pub away_team: String,
    pub date: Option<chrono::NaiveDate>,
    pub time: Option<chrono::NaiveTime>,
    pub report_href: Option<String>,
    pub attendance: Option<i32>,
}

pub fn read_fixure_list(schedule_page: &Document) -> Vec<Fixture> {
    let fixtures = schedule_page
        .find(Name("table").descendant(Name("tbody")))
        .next()
        .unwrap()
        .find(Name("tr"))
        .into_iter()
        .filter(|node| node.text() != "")
        .map(|node| read_fixture(&node))
        .collect();

    fixtures
}

pub fn read_fixture(node: &Node) -> Fixture {
    let fixture_report = match data_stat(node, "match_report").as_str() {
        "Match Report" => Some(
            node.find(Attr("data-stat", "match_report").descendant(Name("a")))
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
                .into(),
        ),
        _ => None,
    };

    let fixture_date_str = data_stat(node, "date");
    let fixture_date = chrono::NaiveDate::from_str(&fixture_date_str).ok();

    let fixture_time_str = format!("{}:{}", data_stat(node, "time"), "00");
    let fixture_time =
        chrono::NaiveTime::parse_from_str(&fixture_time_str, "%H:%M:%S").ok();

    let attendance = if data_stat(node, "attendance").is_empty() {
        None
    } else {
        Some(
            data_stat(node, "attendance")
                .replace(",", "")
                .parse()
                .unwrap(),
        )
    };
    Fixture {
        match_week: data_stat(node, "gameweek").parse().unwrap(),
        home_team: data_stat(node, "squad_a"),
        away_team: data_stat(node, "squad_b"),
        date: fixture_date,
        time: fixture_time,
        report_href: fixture_report,
        attendance,
    }
}

#[derive(Debug)]
pub struct FixtureReport {
    pub home_goals: i16,
    pub away_goals: i16,
    pub home_xg: f32,
    pub away_xg: f32,
    pub home_fixture_record: (i16, i16, i16),
    pub away_fixture_record: (i16, i16, i16),
    pub home_team_stats: TeamStats,
    pub away_team_stats: TeamStats,
    pub home_player_stats: Vec<PlayerStats>,
    pub away_player_stats: Vec<PlayerStats>,
    pub home_gk_stats: Vec<GkStats>,
    pub away_gk_stats: Vec<GkStats>,
    pub home_shot_stats: Vec<ShotStats>,
    pub away_shot_stats: Vec<ShotStats>,
}

pub fn read_fixture_report(page: &Document) -> FixtureReport {
    let mut goals = page
        .find(Attr("class", "score"))
        .into_iter()
        .take(2)
        .map(|node| node.text().trim().parse().unwrap())
        .collect::<Vec<i16>>();
    let away_goals = goals.pop().unwrap();
    let home_goals = goals.pop().unwrap();

    let mut goals_xg = page
        .find(Attr("class", "score_xg"))
        .into_iter()
        .take(2)
        .map(|node| node.text().trim().parse().unwrap())
        .collect::<Vec<f32>>();
    let away_xg = goals_xg.pop().unwrap();
    let home_xg = goals_xg.pop().unwrap();

    let (home_player_stats, away_player_stats) = read_player_stats(page);

    let mut keeper_stats = page
        .find(Name("table"))
        .into_iter()
        .filter(|node| match node.attr("id") {
            None => false,
            Some(val) => val.contains("keeper_stats"),
        })
        .map(|node| {
            node.find(Name("tbody").descendant(Name("tr")))
                .into_iter()
                .map(|node| read_gk_stats(&node))
                .collect()
        })
        .collect::<Vec<Vec<GkStats>>>();

    assert_eq!(keeper_stats.len(), 2);
    let away_gk_stats = keeper_stats.pop().unwrap();
    let home_gk_stats = keeper_stats.pop().unwrap();

    let mut shots_stats = page
        .find(Attr("id", "switcher_shots").descendant(Name("table")))
        .into_iter()
        .skip(1)
        .map(|node| {
            node.find(Name("tbody").descendant(Name("tr")))
                .into_iter()
                .filter(|node| node.text() != "")
                .map(|node| read_shot_stats(&node))
                .collect()
        })
        .collect::<Vec<Vec<ShotStats>>>();

    assert_eq!(shots_stats.len(), 2);
    let away_shot_stats = shots_stats.pop().unwrap();
    let home_shot_stats = shots_stats.pop().unwrap();

    let fixture_records = page.find(Attr("class", "scorebox")).next().unwrap();
    let (home_fixture_record, away_fixture_record) =
        read_fixture_records(&fixture_records);

    let (home_team_stats, away_team_stats) = read_team_stats(&page);
    FixtureReport {
        home_goals,
        away_goals,
        home_xg,
        away_xg,
        home_fixture_record,
        away_fixture_record,
        home_player_stats,
        away_player_stats,
        home_gk_stats,
        away_gk_stats,
        home_shot_stats,
        away_shot_stats,
        home_team_stats,
        away_team_stats,
    }
}

type MatchRec = (i16, i16, i16);
pub fn read_fixture_records(node: &Node) -> (MatchRec, MatchRec) {
    let home = node.children().nth(1).unwrap().children().nth(4).unwrap();
    let away = node.children().nth(3).unwrap().children().nth(4).unwrap();

    let home_rec = home
        .text()
        .replace("(", "")
        .replace(")", "")
        .split("-")
        .map(|val| val.parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    let home_rec = (home_rec[0], home_rec[1], home_rec[2]);

    let away_rec = away
        .text()
        .replace("(", "")
        .replace(")", "")
        .split("-")
        .map(|val| val.parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    let away_rec = (away_rec[0], away_rec[1], away_rec[2]);

    (home_rec, away_rec)
}
