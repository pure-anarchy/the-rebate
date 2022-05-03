use crate::crawler::data_stat;
use select::node::Node;

#[derive(Debug)]
pub struct DefensiveActionStats {
    pub total_tackles: i16,
    pub tackles_won: i16,
    pub tackles_in_defensive_third: i16,
    pub tackles_in_mid_third: i16,
    pub tackles_in_attacking_third: i16,
    pub dribblers_tackled: i16,
    pub dribblers_tackle_attempts: i16,
    pub dribblers_tackle_percent: f32,
    pub dribbled_past: i16,
    pub presses: i16,
    pub presses_successful: i16,
    pub press_success_percent: f32,
    pub presses_in_defensive_third: i16,
    pub presses_in_mid_third: i16,
    pub presses_in_attacking_third: i16,
    pub blocks: i16,
    pub shots_blocked: i16,
    pub on_target_shots_blocked: i16,
    pub passes_blocked: i16,
    pub interceptions: i16,
    pub tackles_plus_interceptions: i16,
    pub clearances: i16,
    pub errors_leading_to_shot: i16,
}

pub fn read_def_action_stats(node: &Node) -> DefensiveActionStats {
    DefensiveActionStats {
        total_tackles: data_stat(node, "tackles").parse().unwrap_or(0),
        tackles_won: data_stat(node, "tackles_won").parse().unwrap_or(0),
        tackles_in_defensive_third: data_stat(node, "tackles_def_3rd")
            .parse()
            .unwrap_or(0),
        tackles_in_mid_third: data_stat(node, "tackles_mid_3rd")
            .parse()
            .unwrap_or(0),
        tackles_in_attacking_third: data_stat(node, "tackles_att_3rd")
            .parse()
            .unwrap_or(0),
        dribblers_tackled: data_stat(node, "dribble_tackles")
            .parse()
            .unwrap_or(0),
        dribblers_tackle_attempts: data_stat(node, "dribbles_vs")
            .parse()
            .unwrap_or(0),
        dribblers_tackle_percent: data_stat(node, "dribble_tackles_pct")
            .parse()
            .unwrap_or(0.0),
        dribbled_past: data_stat(node, "dribbled_past").parse().unwrap_or(0),
        presses: data_stat(node, "pressures").parse().unwrap_or(0),
        presses_successful: data_stat(node, "pressure_regains")
            .parse()
            .unwrap_or(0),
        press_success_percent: data_stat(node, "pressure_regain_pct")
            .parse()
            .unwrap_or(0.0),
        presses_in_defensive_third: data_stat(node, "pressures_def_3rd")
            .parse()
            .unwrap_or(0),
        presses_in_mid_third: data_stat(node, "pressures_att_3rd")
            .parse()
            .unwrap_or(0),
        presses_in_attacking_third: data_stat(node, "pressures_mid_3rd")
            .parse()
            .unwrap_or(0),
        blocks: data_stat(node, "blocks").parse().unwrap_or(0),
        shots_blocked: data_stat(node, "blocked_shots").parse().unwrap_or(0),
        on_target_shots_blocked: data_stat(node, "blocked_shots_saves")
            .parse()
            .unwrap_or(0),
        passes_blocked: data_stat(node, "blocked_passes").parse().unwrap_or(0),
        interceptions: data_stat(node, "interceptions").parse().unwrap_or(0),
        tackles_plus_interceptions: data_stat(node, "tackles_interceptions")
            .parse()
            .unwrap_or(0),
        clearances: data_stat(node, "clearances").parse().unwrap_or(0),
        errors_leading_to_shot: data_stat(node, "errors").parse().unwrap_or(0),
    }
}
