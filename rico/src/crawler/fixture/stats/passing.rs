use crate::crawler::data_stat;
use select::node::Node;

use crate::error::Errable;

#[derive(Debug)]
pub struct PassingStat {
    pub passes_completed: i16,
    pub passes_attempted: i16,
    pub pass_accuracy: f32,
    pub total_pass_distance: i16,
    pub total_pass_progessive_distance: i16,
    pub short_range_passes_completed: i16,
    pub short_range_passes_attempted: i16,
    pub short_range_pass_accuracy: f32,
    pub mid_range_passes_completed: i16,
    pub mid_range_passes_attempted: i16,
    pub mid_range_pass_accuracy: f32,
    pub long_range_passes_completed: i16,
    pub long_range_passes_attempted: i16,
    pub long_range_pass_accuracy: f32,
    pub assists: i16,
    pub expected_assists: f32,
    pub key_passes: i16,
    pub passes_completed_in_final_third: i16,
    pub passes_completed_in_pen_area: i16,
    pub crosses_completed_in_pen_area: i16,
    pub progressive_passes: i16,
}

pub fn read_passing_stats(node: &Node) -> PassingStat {
    PassingStat {
        passes_completed: data_stat(node, "passes_completed")
            .parse()
            .unwrap_or(0),
        passes_attempted: data_stat(node, "passes").parse().unwrap_or(0),
        pass_accuracy: data_stat(node, "passes_pct").parse().unwrap_or(0.0),
        total_pass_distance: data_stat(node, "passes_total_distance")
            .parse()
            .unwrap_or(0),
        total_pass_progessive_distance: data_stat(
            node,
            "passes_progressive_distance",
        )
        .parse()
        .unwrap_or(0),
        short_range_passes_completed: data_stat(node, "passes_completed_short")
            .parse()
            .unwrap_or(0),
        short_range_passes_attempted: data_stat(node, "passes_short")
            .parse()
            .unwrap_or(0),
        short_range_pass_accuracy: data_stat(node, "passes_pct_short")
            .parse()
            .unwrap_or(0.0),
        mid_range_passes_completed: data_stat(node, "passes_completed_medium")
            .parse()
            .unwrap_or(0),
        mid_range_passes_attempted: data_stat(node, "passes_medium")
            .parse()
            .unwrap_or(0),
        mid_range_pass_accuracy: data_stat(node, "passes_pct_medium")
            .parse()
            .unwrap_or(0.0),
        long_range_passes_completed: data_stat(node, "passes_completed_long")
            .parse()
            .unwrap_or(0),
        long_range_passes_attempted: data_stat(node, "passes_long")
            .parse()
            .unwrap_or(0),
        long_range_pass_accuracy: data_stat(node, "passes_pct_long")
            .parse()
            .unwrap_or(0.0),
        assists: data_stat(node, "assists").parse().unwrap_or(0),
        expected_assists: data_stat(node, "xa").parse().unwrap_or(0.0),
        key_passes: data_stat(node, "assisted_shots").parse().unwrap_or(0),
        passes_completed_in_final_third: data_stat(
            node,
            "passes_into_final_third",
        )
        .parse()
        .unwrap_or(0),
        passes_completed_in_pen_area: data_stat(
            node,
            "passes_into_penalty_area",
        )
        .parse()
        .unwrap_or(0),
        crosses_completed_in_pen_area: data_stat(
            node,
            "crosses_into_penalty_area",
        )
        .parse()
        .unwrap_or(0),
        progressive_passes: data_stat(node, "progressive_passes")
            .parse()
            .unwrap_or(0),
    }
}
