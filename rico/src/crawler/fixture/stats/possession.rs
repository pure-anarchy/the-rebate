use crate::crawler::data_stat;
use select::node::Node;

#[derive(Debug)]
pub struct PossessionStat {
    pub total_touches: i16,
    pub touches_in_def_pen_area: i16,
    pub touches_in_defensive_third: i16,
    pub touches_in_mid_third: i16,
    pub touches_in_final_third: i16,
    pub touches_in_attacking_pen_area: i16,
    pub live_ball_touches: i16,
    pub dribbles_successful: i16,
    pub dribbles_attepmted: i16,
    pub dribble_success_percent: f32,
    pub num_of_players_dribbled_past: i16,
    pub nutmegs: i16,
    pub carries: i16,
    pub total_carry_distance: i16,
    pub progressive_carry_distance: i16,
    pub progressive_carries: i16,
    pub carries_into_final_third: i16,
    pub carries_into_penalty_box: i16,
    pub miscontrols_of_the_ball: i16,
    pub dispossessed: i16,
    pub passes_received_as_target_player: i16,
    pub passes_received: i16,
    pub pass_received_percent: f32,
    pub progressive_passes_received: i16,
}

pub fn read_possession_stats(node: &Node) -> PossessionStat {
    PossessionStat {
        total_touches: data_stat(node, "touches").parse().unwrap_or(0),
        touches_in_def_pen_area: data_stat(node, "touches_def_pen_area")
            .parse()
            .unwrap_or(0),
        touches_in_defensive_third: data_stat(node, "touches_def_3rd")
            .parse()
            .unwrap_or(0),
        touches_in_mid_third: data_stat(node, "touches_mid_3rd")
            .parse()
            .unwrap_or(0),
        touches_in_final_third: data_stat(node, "touches_att_3rd")
            .parse()
            .unwrap_or(0),
        touches_in_attacking_pen_area: data_stat(node, "touches_att_pen_area")
            .parse()
            .unwrap_or(0),
        live_ball_touches: data_stat(node, "touches_live_ball")
            .parse()
            .unwrap_or(0),

        dribbles_successful: data_stat(node, "dribbles_completed")
            .parse()
            .unwrap_or(0),
        dribbles_attepmted: data_stat(node, "dribbles").parse().unwrap_or(0),
        dribble_success_percent: data_stat(node, "dribbles_completed_pct")
            .parse()
            .unwrap_or(0.0),
        num_of_players_dribbled_past: data_stat(node, "players_dribbled_past")
            .parse()
            .unwrap_or(0),
        nutmegs: data_stat(node, "nutmegs").parse().unwrap_or(0),
        carries: data_stat(node, "carries").parse().unwrap_or(0),
        total_carry_distance: data_stat(node, "carry_distance")
            .parse()
            .unwrap_or(0),
        progressive_carry_distance: data_stat(
            node,
            "carry_progressive_distance",
        )
        .parse()
        .unwrap_or(0),
        progressive_carries: data_stat(node, "progressive_carries")
            .parse()
            .unwrap_or(0),
        carries_into_final_third: data_stat(node, "carries_into_final_third")
            .parse()
            .unwrap_or(0),
        carries_into_penalty_box: data_stat(node, "carries_into_penalty_area")
            .parse()
            .unwrap_or(0),
        miscontrols_of_the_ball: data_stat(node, "miscontrols")
            .parse()
            .unwrap_or(0),
        dispossessed: data_stat(node, "dispossessed").parse().unwrap_or(0),
        passes_received_as_target_player: data_stat(node, "pass_targets")
            .parse()
            .unwrap_or(0),
        passes_received: data_stat(node, "passes_received")
            .parse()
            .unwrap_or(0),
        pass_received_percent: data_stat(node, "passes_received_pct")
            .parse()
            .unwrap_or(0.0),
        progressive_passes_received: data_stat(
            node,
            "progressive_passes_received",
        )
        .parse()
        .unwrap_or(0),
    }
}
