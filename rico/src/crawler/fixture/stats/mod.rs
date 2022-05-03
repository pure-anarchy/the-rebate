pub mod def_actions;
pub mod goalkeeper;
pub mod misc;
pub mod passing;
pub mod passtypes;
pub mod possession;
pub mod shots;
pub mod team;

use select::{
    document::Document,
    node::Node,
    predicate::{Name, Predicate},
};

use crate::{crawler::data_stat, error::Errable};

use self::{
    def_actions::{read_def_action_stats, DefensiveActionStats},
    misc::{read_misc_stats, MiscellaneousStats},
    passing::{read_passing_stats, PassingStat},
    passtypes::{read_pass_type_stats, PassTypeStat},
    possession::{read_possession_stats, PossessionStat},
};

pub fn read_player_stats(
    page: &Document,
) -> (Vec<PlayerStats>, Vec<PlayerStats>) {
    let home_stat_nodes = page
        .find(Name("table"))
        .into_iter()
        .filter(|node| match node.attr("class") {
            None => false,
            Some(val) => val.contains("stats_table"),
        })
        .skip(1)
        .take(5)
        .map(|node| {
            node.find(Name("tbody").descendant(Name("tr")))
                .into_iter()
                .collect()
        })
        .collect::<Vec<Vec<Node>>>();

    let mut home_player_stats = vec![];

    for idx in 0..home_stat_nodes[0].len() {
        home_player_stats.push(PlayerStats {
            player_info: read_player_info(&home_stat_nodes[0][idx]),
            passing: read_passing_stats(&home_stat_nodes[0][idx]),
            passtypes: read_pass_type_stats(&home_stat_nodes[1][idx]),

            def_actions: read_def_action_stats(&home_stat_nodes[2][idx]),
            possession: read_possession_stats(&home_stat_nodes[3][idx]),
            misc: read_misc_stats(&home_stat_nodes[4][idx]),
        })
    }

    let away_stat_nodes = page
        .find(Name("table"))
        .into_iter()
        .filter(|node| match node.attr("class") {
            None => false,
            Some(val) => val.contains("stats_table"),
        })
        .skip(8)
        .take(5)
        .map(|node| {
            node.find(Name("tbody").descendant(Name("tr")))
                .into_iter()
                .collect()
        })
        .collect::<Vec<Vec<Node>>>();

    let mut away_player_stats = vec![];

    for idx in 0..away_stat_nodes[0].len() {
        away_player_stats.push(PlayerStats {
            player_info: read_player_info(&away_stat_nodes[0][idx]),
            passing: read_passing_stats(&away_stat_nodes[0][idx]),
            passtypes: read_pass_type_stats(&away_stat_nodes[1][idx]),

            def_actions: read_def_action_stats(&away_stat_nodes[2][idx]),
            possession: read_possession_stats(&away_stat_nodes[3][idx]),
            misc: read_misc_stats(&away_stat_nodes[4][idx]),
        })
    }

    (home_player_stats, away_player_stats)
}

#[derive(Debug)]
pub struct PlayerStats {
    pub player_info: PlayerInfo,
    pub passing: PassingStat,
    pub misc: MiscellaneousStats,
    pub def_actions: DefensiveActionStats,
    pub possession: PossessionStat,
    pub passtypes: PassTypeStat,
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub player_name: String,
    pub jersey_number: i16,
    pub age: (i16, i16),
    pub nationality: String,
    pub position: String,
    pub minutes_played: String,
}

fn read_player_info(node: &Node) -> PlayerInfo {
    let age_str = data_stat(node, "age");
    let age_str: Vec<&str> = age_str.split("-").into_iter().collect();

    PlayerInfo {
        player_name: data_stat(node, "player"),
        jersey_number: data_stat(node, "shirtnumber").parse().unwrap_or(0),
        age: (
            age_str[0].parse().unwrap_or(0),
            age_str[1].parse().unwrap_or(0),
        ),
        nationality: data_stat(node, "nationality"),
        position: data_stat(node, "position"),
        minutes_played: data_stat(node, "minutes"),
    }
}
