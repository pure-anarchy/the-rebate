use select::node::Node;

use crate::{crawler::data_stat, error::Errable};

#[derive(Debug)]
pub struct PassTypeStat {
    pub passes_attempted: i16,
    pub live_ball_passes: i16,
    pub dead_ball_passes: i16,
    pub fk_passes: i16,
    pub through_ball_passes: i16,
    pub passes_made_under_press: i16,
    pub lateral_passes: i16,
    pub crosses: i16,
    pub corner_kicks: i16,
    pub inswinging_corner: i16,
    pub outswinging_corner: i16,
    pub straight_corners: i16,
    pub ground_passes: i16,
    pub low_alt_passes: i16,
    pub high_alt_passes: i16,
    pub left_foot_passes: i16,
    pub right_foot_passes: i16,
    pub header_passes: i16,
    pub throw_ins: i16,
    pub other_bodypart_passes: i16,
    pub passes_completed: i16,
    pub offside_passes: i16,
    pub out_of_bounds_passes: i16,
    pub intercepted_passes: i16,
    pub passes_blocked: i16,
}

pub fn read_pass_type_stats(node: &Node) -> PassTypeStat {
    PassTypeStat {
        passes_attempted: data_stat(node, "passes").parse().unwrap_or(0),
        live_ball_passes: data_stat(node, "passes_live").parse().unwrap_or(0),
        dead_ball_passes: data_stat(node, "passes_dead").parse().unwrap_or(0),
        fk_passes: data_stat(node, "passes_free_kicks").parse().unwrap_or(0),

        through_ball_passes: data_stat(node, "through_balls")
            .parse()
            .unwrap_or(0),
        lateral_passes: data_stat(node, "passes_switches").parse().unwrap_or(0),
        crosses: data_stat(node, "crosses").parse().unwrap_or(0),
        corner_kicks: data_stat(node, "corner_kicks").parse().unwrap_or(0),

        inswinging_corner: data_stat(node, "corner_kicks_in")
            .parse()
            .unwrap_or(0),
        passes_blocked: data_stat(node, "passes_blocked").parse().unwrap_or(0),
        outswinging_corner: data_stat(node, "corner_kicks_out")
            .parse()
            .unwrap_or(0),
        ground_passes: data_stat(node, "passes_ground").parse().unwrap_or(0),

        low_alt_passes: data_stat(node, "passes_low").parse().unwrap_or(0),
        high_alt_passes: data_stat(node, "passes_high").parse().unwrap_or(0),
        left_foot_passes: data_stat(node, "passes_left_foot")
            .parse()
            .unwrap_or(0),
        right_foot_passes: data_stat(node, "passes_right_foot")
            .parse()
            .unwrap_or(0),

        header_passes: data_stat(node, "passes_head").parse().unwrap_or(0),
        throw_ins: data_stat(node, "throw_ins").parse().unwrap_or(0),
        passes_completed: data_stat(node, "passes_completed")
            .parse()
            .unwrap_or(0),
        offside_passes: data_stat(node, "passes_offsides").parse().unwrap_or(0),
        out_of_bounds_passes: data_stat(node, "passes_oob")
            .parse()
            .unwrap_or(0),

        passes_made_under_press: data_stat(node, "passes_pressure")
            .parse()
            .unwrap_or(0),
        intercepted_passes: data_stat(node, "passes_intercepted")
            .parse()
            .unwrap_or(0),
        straight_corners: data_stat(node, "corner_kicks_straight")
            .parse()
            .unwrap_or(0),
        other_bodypart_passes: data_stat(node, "passes_other_body")
            .parse()
            .unwrap_or(0),
    }
}
