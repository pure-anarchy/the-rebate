use crate::crawler::data_stat;
use select::node::Node;

#[derive(Debug)]
pub struct ShotStats {
    pub player_name: String,
    pub shot_minute: i16,
    pub shot_outcome: String,
    pub distance_from_goal: i16,
    pub body_part: String,
    pub first_shot_creator: Option<String>,
    pub first_shot_creating_event: Option<String>,
    pub second_shot_creator: Option<String>,
    pub second_shot_creating_event: Option<String>,
}

pub fn read_shot_stats(node: &Node) -> ShotStats {
    let first_shot_creator = match data_stat(node, "sca_1_player").as_str() {
        "" => None,
        val => Some(val.into()),
    };

    let first_shot_creating_event = match data_stat(node, "sca_1_type").as_str()
    {
        "" => None,
        val => Some(val.into()),
    };

    let second_shot_creator = match data_stat(node, "sca_2_player").as_str() {
        "" => None,
        val => Some(val.into()),
    };

    let second_shot_creating_event =
        match data_stat(node, "sca_2_type").as_str() {
            "" => None,
            val => Some(val.into()),
        };
    ShotStats {
        player_name: data_stat(node, "player"),
        shot_minute: data_stat(node, "minute").parse().unwrap_or(0),
        shot_outcome: data_stat(node, "outcome"),
        distance_from_goal: data_stat(node, "distance").parse().unwrap_or(0),
        body_part: data_stat(node, "body_part"),
        first_shot_creator,
        first_shot_creating_event,
        second_shot_creator,
        second_shot_creating_event,
    }
}
