create type age as (
    years smallint,
    days smallint
);

create table player_stats_tbl (
    fixture_id int not null,
    team_id int not null,
    player_name varchar(60) not null,
    player_national varchar(6) not null,
    position varchar(6) not null,
    jersey_number smallint not null,
    player_age age not null,
    minutes_played smallint not null,
    goals_scored smallint not null,
    assists smallint not null,
    penalties_scored smallint not null,
    penalties_played smallint not null,
    penalties_won smallint not null,
    penalties_conceded smallint not null,
    own_goals smallint not null,
    loose_ball_recoveries smallint not null,
    aerials_won smallint not null,
    aerials_lost smallint not null,
    aerial_win_ratio smallint not null,
    shots_total smallint not null,
    yellow_cards smallint not null,
    second_yellow_cards smallint not null,
    red_cards smallint not null,
    fouls_committed smallint not null,
    fouls_drawn smallint not null,
    offsides smallint not null,
    touches smallint not null,
    touches_in_defensive_pen_box smallint not null,
    touches_in_defensive_third smallint not null,
    touches_in_mid_third smallint not null,
    touches_in_attacking_third smallint not null,
    live_ball_touches smallint,
    presses smallint not null,
    successful_presses smallint not null,
    press_success_ratio smallint not null,
    presses_in_defensive_third smallint not null,
    presses_in_mid_third smallint not null,
    presses_in_attacking_third smallint not null,
    tackles smallint not null,
    tackles_won smallint not null,
    tackles_in_defensive_third smallint not null,
    tackles_in_mid_third smallint not null,
    tackles_in_attacking_third smallint not null,
    tackle_attempts smallint not null,
    dribblers_tackled smallint not null,
    dribblers_tackle_win_ratio decimal(4, 2) not null,
    dribbled_past smallint not null,
    interceptions smallint not null,
    blocks smallint not null,
    shots_blocked smallint not null,
    shots_on_target_blocked smallint not null,
    expected_goals smallint not null,
    expected_non_penalty_goals smallint not null,
    expected_assists smallint not null,
    shot_creating_actions smallint not null,
    goal_creating_actions smallint not null,
    passes_completed smallint not null,
    passes_attempted smallint not null,
    pass_accuracy decimal(4, 2) not null,
    total_pass_distance smallint not null,
    progressive_passes smallint not null,
    progressive_pass_distance smallint not null,
    short_passes_completed smallint not null,
    short_passes_attempted smallint not null,
    short_pass_accuracy decimal(4, 2) not null,
    med_passes_completed smallint not null,
    med_passes_attempted smallint not null,
    med_pass_accuracy decimal(4, 2) not null,
    long_passes_completed smallint not null,
    long_passes_attempted smallint not null,
    long_pass_accuracy decimal(4, 2) not null,
    key_passes smallint not null,
    passes_into_final_third smallint not null,
    passes_into_penalty_area smallint not null,
    crosses smallint not null,
    corner_kicks smallint not null,
    in_swinging_corners smallint not null,
    out_swinging_corners smallint not null,
    straight_corners smallint not null,
    ground_passes smallint not null,
    mid_altitude_passes smallint not null,
    high_altitude_passes smallint not null,
    crosses_into_penalty_area smallint not null,
    live_ball_passes smallint not null,
    dead_ball_passes smallint not null,
    free_kick_passes smallint not null,
    through_ball_passes smallint not null,
    passes_under_press smallint not null,
    lateral_passes smallint not null,
    left_foot_passes smallint not null,
    right_foot_passes smallint not null,
    header_passes smallint not null,
    throw_ins smallint not null,
    misc_body_part_passes smallint not null,
    passes_to_offside_player smallint not null,
    passes_out_of_bounds smallint not null,
    passes_intercepted smallint not null,
    passes_blocked smallint not null,
    carries smallint not null,
    progressive_carries smallint not null,
    carry_distance smallint not null,
    progressive_carry_distance smallint not null,
    carries_into_final_third smallint not null,
    miscontrols smallint not null,
    dispossessed smallint not null,
    passes_attempted_to_op smallint not null,
    passes_received smallint not null,
    pass_receiving_ratio decimal(4, 2) not null,
    progressive_passes_received smallint not null,
    carries_into_penalty_area smallint not null,
    dribbles_successful smallint not null,
    dribbles_attempted smallint not null,
    dribble_success_ratio decimal(4, 2) not null,
    players_dribbled_past smallint not null,
    nutmegs smallint not null,
    clearances smallint not null,
    errors_leading_to_goal smallint not null,
    tackle_attempts_and_clearances smallint not null,
    ts_player_name tsvector,




    constraint fixture_id_is_not_valid 
	foreign key (fixture_id) references fixture_tbl(fixture_id),

    constraint fk_team_is_id_not_valid
	foreign key (team_id) references team_tbl(team_id)

);



create function fill_player_stats_player_name_text_search_column()
  returns trigger as
$$
begin
  new.ts_player_name := to_tsvector(new.player_name);
  return new;
end;    
$$ language plpgsql;

create trigger update_shot_stat_player_name_tokens
  before update or insert on player_stats_tbl
  for each row execute procedure fill_player_stats_player_name_text_search_column();


create 
    index ts_player_stats_player_name_idx 
    on player_stats_tbl using gin (ts_player_name);
