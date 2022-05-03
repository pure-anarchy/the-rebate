use crate::crawler::data_stat;
use select::node::Node;

#[derive(Debug)]
pub struct MiscellaneousStats {
    pub yellow_cards: i16,
    pub red_cards: i16,
    pub second_yellow_cards: i16,
    pub fouls_committed: i16,
    pub fouls_drawn: i16,
    pub offsides: i16,
    pub crosses: i16,
    pub interceptions: i16,
    pub tackles_won: i16,
    pub pens_won: i16,
    pub pens_conceded: i16,
    pub own_goals: i16,
    pub ball_recoveries: i16,
    pub aerials_won: i16,
    pub aerials_lost: i16,
    pub aerials_win_percent: f32,
}

pub fn read_misc_stats(node: &Node) -> MiscellaneousStats {
    MiscellaneousStats {
        yellow_cards: data_stat(node, "cards_yellow").parse().unwrap_or(0),
        red_cards: data_stat(node, "cards_red").parse().unwrap_or(0),
        fouls_committed: data_stat(node, "fouls").parse().unwrap_or(0),

        fouls_drawn: data_stat(node, "fouled").parse().unwrap_or(0),
        offsides: data_stat(node, "offsides").parse().unwrap_or(0),
        crosses: data_stat(node, "crosses").parse().unwrap_or(0),

        interceptions: data_stat(node, "interceptions").parse().unwrap_or(0),
        tackles_won: data_stat(node, "tackles_won").parse().unwrap_or(0),
        pens_won: data_stat(node, "pens_won").parse().unwrap_or(0),

        pens_conceded: data_stat(node, "pens_conceded").parse().unwrap_or(0),
        own_goals: data_stat(node, "own_goals").parse().unwrap_or(0),
        ball_recoveries: data_stat(node, "ball_recoveries")
            .parse()
            .unwrap_or(0),

        aerials_won: data_stat(node, "aerials_won").parse().unwrap_or(0),
        aerials_lost: data_stat(node, "aerials_lost").parse().unwrap_or(0),
        aerials_win_percent: data_stat(node, "aerials_won_pct")
            .parse()
            .unwrap_or(0.0),

        second_yellow_cards: data_stat(node, "cards_yellow_red")
            .parse()
            .unwrap_or(0),
    }
}
