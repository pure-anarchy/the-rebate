use select::node::Node;
use select::predicate::{Name, Predicate};
use select::{document::Document, predicate::Attr};

#[derive(Debug)]
pub struct TeamStats {
    pub formation: String,
    pub possession: f32,
    pub total_passes: i16,
    pub accurate_passes: i16,
    pub shots_taken: i16,
    pub shots_on_target: i16,
    pub gk_saves: i16,
    pub yellow_cards: i16,
    pub second_yellows: i16,
    pub red_cards: i16,
    pub fouls_committed: i16,
    pub corners_taken: i16,
    pub crosses: i16,
    pub total_touches: i16,
    pub total_tackles: i16,
    pub interceptions: i16,
    pub aerials_won: i16,
    pub clearances: i16,
    pub offsides: i16,
    pub goal_kicks: i16,
    pub throw_ins: i16,
}

pub fn nth_descendant(node: &Node, nth: usize) -> String {
    node.descendants().nth(nth).unwrap().text().trim().into()
}

pub fn read_team_stats(match_page: &Document) -> (TeamStats, TeamStats) {
    let lineups = match_page
        .find(Attr("class", "lineup"))
        .into_iter()
        .collect::<Vec<Node>>();

    let home_formation = lineups[0]
        .find(Name("tbody").descendant(Name("tr")))
        .into_iter()
        .nth(0)
        .unwrap()
        .text()
        .replace(")", "")
        .split("(")
        .skip(1)
        .take(1)
        .collect::<String>();

    let away_formation = lineups[1]
        .find(Name("tbody").descendant(Name("tr")))
        .into_iter()
        .nth(0)
        .unwrap()
        .text()
        .replace(")", "")
        .split("(")
        .skip(1)
        .take(1)
        .collect::<String>();

    let team_stats_table = match_page
        .find(Attr("id", "team_stats").descendant(Name("tbody")))
        .next()
        .unwrap()
        .find(Name("tr"))
        .into_iter()
        .collect::<Vec<Node>>();

    let possession_stats = team_stats_table[2]
        .find(Name("td"))
        .into_iter()
        .map(|node| node.text().trim().replace("%", "").parse().unwrap())
        .collect::<Vec<f32>>();

    let pass_stats = team_stats_table[4]
        .find(Name("td"))
        .map(|node| node.text().replace("\u{a0}", " ").trim().to_owned())
        .collect::<Vec<String>>();

    let home_pass_stats =
        &pass_stats[0].split(" — ").take(1).collect::<String>();
    let home_pass_stats = home_pass_stats.split(" of ").collect::<Vec<&str>>();

    let away_pass_stats = &pass_stats[1]
        .split(" — ")
        .skip(1)
        .take(1)
        .collect::<String>();

    let away_pass_stats = away_pass_stats.split(" of ").collect::<Vec<&str>>();

    let shots_stats = team_stats_table[6]
        .find(Name("td"))
        .into_iter()
        .map(|node| node.text().replace("\u{a0}", " ").trim().to_owned())
        .collect::<Vec<String>>();

    let home_shot_stats = &shots_stats[1]
        .split(" — ")
        .skip(1)
        .take(1)
        .collect::<String>();
    let home_shot_stats = home_shot_stats.split(" of ").collect::<Vec<&str>>();

    let away_shot_stats = &shots_stats[1]
        .split(" — ")
        .skip(1)
        .take(1)
        .collect::<String>();
    let away_shot_stats = away_shot_stats.split(" of ").collect::<Vec<&str>>();

    let gk_stats = team_stats_table[8]
        .find(Name("td"))
        .into_iter()
        .collect::<Vec<Node>>();

    let home_gk_stats = gk_stats[0]
        .text()
        .trim()
        .replace("\u{a0}", " ")
        .split(" — ")
        .take(1)
        .collect::<String>();

    let home_gk_stats = home_gk_stats.split(" of ").collect::<Vec<&str>>();

    let away_gk_stats = gk_stats[1]
        .text()
        .trim()
        .replace("\u{a0}", " ")
        .split(" — ")
        .skip(1)
        .take(1)
        .collect::<String>();

    let away_gk_stats = away_gk_stats.split(" of ").collect::<Vec<&str>>();

    let cards = team_stats_table[10]
        .find(Name("td"))
        .into_iter()
        .collect::<Vec<Node>>();

    let home_yellows = cards[0].find(Attr("class", "yellow_card")).count();
    let home_reds = cards[0].find(Attr("class", "red_card")).count();

    let home_sec_yellows =
        cards[0].find(Attr("class", "yellow_red_card")).count();

    let away_yellows = cards[1].find(Attr("class", "yellow_card")).count();
    let away_reds = cards[1].find(Attr("class", "red_card")).count();

    let away_sec_yellows =
        cards[1].find(Attr("class", "yellow_red_card")).count();

    let team_stats_ext = match_page
        .find(Attr("id", "team_stats_extra"))
        .next()
        .unwrap();

    let home_stats = TeamStats {
        formation: home_formation,
        possession: possession_stats[0],
        total_passes: *&home_pass_stats[1].parse().unwrap(),
        accurate_passes: *&home_pass_stats[0].parse().unwrap(),
        shots_taken: *&home_shot_stats[1].parse().unwrap(),
        shots_on_target: *&home_shot_stats[0].parse().unwrap(),
        gk_saves: *&home_gk_stats[0].parse().unwrap(),
        yellow_cards: home_yellows as i16,
        second_yellows: home_sec_yellows as i16,
        red_cards: home_reds as i16,

        fouls_committed: nth_descendant(&team_stats_ext, 11).parse().unwrap(),
        corners_taken: nth_descendant(&team_stats_ext, 17).parse().unwrap(),
        crosses: nth_descendant(&team_stats_ext, 24).parse().unwrap(),
        total_touches: nth_descendant(&team_stats_ext, 32).parse().unwrap(),
        total_tackles: nth_descendant(&team_stats_ext, 48).parse().unwrap(),
        interceptions: nth_descendant(&team_stats_ext, 55).parse().unwrap(),
        aerials_won: nth_descendant(&team_stats_ext, 62).parse().unwrap(),
        clearances: nth_descendant(&team_stats_ext, 69).parse().unwrap(),
        offsides: nth_descendant(&team_stats_ext, 86).parse().unwrap(),
        goal_kicks: nth_descendant(&team_stats_ext, 93).parse().unwrap(),
        throw_ins: nth_descendant(&team_stats_ext, 101).parse().unwrap(),
    };

    let away_stats = TeamStats {
        formation: away_formation,
        possession: possession_stats[1],
        total_passes: *&away_pass_stats[1].parse().unwrap(),
        accurate_passes: *&away_pass_stats[0].parse().unwrap(),
        shots_taken: *&away_shot_stats[1].parse().unwrap(),
        shots_on_target: *&away_shot_stats[0].parse().unwrap(),
        gk_saves: away_gk_stats[0].parse().unwrap(),
        yellow_cards: away_yellows as i16,
        second_yellows: away_sec_yellows as i16,
        red_cards: away_reds as i16,

        fouls_committed: nth_descendant(&team_stats_ext, 15).parse().unwrap(),
        corners_taken: nth_descendant(&team_stats_ext, 21).parse().unwrap(),
        crosses: nth_descendant(&team_stats_ext, 28).parse().unwrap(),
        total_touches: nth_descendant(&team_stats_ext, 36).parse().unwrap(),
        total_tackles: nth_descendant(&team_stats_ext, 52).parse().unwrap(),
        interceptions: nth_descendant(&team_stats_ext, 59).parse().unwrap(),
        aerials_won: nth_descendant(&team_stats_ext, 66).parse().unwrap(),
        clearances: nth_descendant(&team_stats_ext, 73).parse().unwrap(),
        offsides: nth_descendant(&team_stats_ext, 90).parse().unwrap(),
        goal_kicks: nth_descendant(&team_stats_ext, 97).parse().unwrap(),
        throw_ins: nth_descendant(&team_stats_ext, 104).parse().unwrap(),
    };

    (home_stats, away_stats)
}
