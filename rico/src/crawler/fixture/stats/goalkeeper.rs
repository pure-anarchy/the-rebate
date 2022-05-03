use select::node::Node;

use crate::crawler::data_stat;

#[derive(Debug)]
pub struct GkStats {
    pub gk_name: String,
    pub nationality: String,
    pub age: (i16, i16),
    pub minutes_played: i16,
    pub shots_on_target_faced: i16,
    pub goals_conceded: i16,
    pub saves: i16,
    pub save_ratio: f32,
    pub post_shot_xg: f32,
    pub launch_passes_completed: i16,
    pub launch_passes_attempted: i16,
    pub launch_pass_accuracy: f32,
    pub passes_attempted: i16,
    pub throws_attempted: i16,
    pub launch_to_passes_ratio: f32,
    pub avg_pass_length: f32,
    pub goal_kicks_attempted: i16,
    pub goal_kick_launch_ratio: f32,
    pub goal_kick_avg_length: f32,
    pub crosses_into_penalty_area_faced: i16,
    pub crosses_into_penalty_area_stopped: i16,
    pub crosses_into_penalty_area_stop_ratio: f32,
    pub defensive_actions_outside_penalty_area: i16,
    pub avg_distance_from_goal_per_defensive_action: f32,
}

pub fn read_gk_stats(node: &Node) -> GkStats {
    let age_str = data_stat(node, "age");
    let age_str: Vec<&str> = age_str.split("-").into_iter().collect();

    GkStats {
        gk_name: data_stat(node, "player"),
        nationality: data_stat(node, "nationality"),
        age: (
            age_str[0].parse().unwrap_or(0),
            age_str[1].parse().unwrap_or(0),
        ),

        minutes_played: data_stat(node, "minutes").parse().unwrap_or(0),
        saves: data_stat(node, "saves").parse().unwrap_or(0),
        save_ratio: data_stat(node, "save_pct").parse().unwrap_or(0.0),
        post_shot_xg: data_stat(node, "psxg_gk").parse().unwrap_or(0.0),

        launch_passes_attempted: data_stat(node, "passes_launched_gk")
            .parse()
            .unwrap_or(0),
        launch_pass_accuracy: data_stat(node, "passes_pct_launched_gk")
            .parse()
            .unwrap_or(0.0),
        passes_attempted: data_stat(node, "passes_gk").parse().unwrap_or(0),

        throws_attempted: data_stat(node, "passes_throws_gk")
            .parse()
            .unwrap_or(0),
        launch_to_passes_ratio: data_stat(node, "pct_passes_launched_gk")
            .parse()
            .unwrap_or(0.0),
        avg_pass_length: data_stat(node, "passes_length_avg_gk")
            .parse()
            .unwrap_or(0.0),
        goal_kicks_attempted: data_stat(node, "goal_kicks")
            .parse()
            .unwrap_or(0),
        goal_kick_launch_ratio: data_stat(node, "pct_goal_kicks_launched")
            .parse()
            .unwrap_or(0.0),
        goal_kick_avg_length: data_stat(node, "goal_kick_length_avg")
            .parse()
            .unwrap_or(0.0),

        crosses_into_penalty_area_faced: data_stat(node, "crosses_gk")
            .parse()
            .unwrap_or(0),
        crosses_into_penalty_area_stopped: data_stat(
            node,
            "crosses_stopped_gk",
        )
        .parse()
        .unwrap_or(0),

        crosses_into_penalty_area_stop_ratio: data_stat(
            node,
            "crosses_stopped_pct_gk",
        )
        .parse()
        .unwrap_or(0.0),

        defensive_actions_outside_penalty_area: data_stat(
            node,
            "def_actions_outside_pen_area_gk",
        )
        .parse()
        .unwrap_or(0),

        avg_distance_from_goal_per_defensive_action: data_stat(
            node,
            "avg_distance_def_actions_gk",
        )
        .parse()
        .unwrap_or(0.0),

        launch_passes_completed: data_stat(
            node,
            "passes_completed_launched_gk",
        )
        .parse()
        .unwrap_or(0),

        shots_on_target_faced: data_stat(node, "shots_on_target_against")
            .parse()
            .unwrap_or(0),
        goals_conceded: data_stat(node, "goals_against_gk")
            .parse()
            .unwrap_or(0),
    }
}
