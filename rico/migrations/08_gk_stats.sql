create table gk_stats_tbl (
    fixture_id smallint not null,
    team_id smallint not null,
    gk_name varchar(80) not null,
    gk_national varchar(3) not null,
    gk_age age not null,
    minutes_played smallint not null,
    shots_on_target_faced smallint not null,
    goals_conceded smallint not null,
    saves smallint not null,
    save_ratio decimal(4, 2) not null,
    post_shot_xg decimal(6, 2) not null,
    launch_passes_completed smallint not null,
    launch_passes_attempted smallint not null,
    launch_pass_accuracy decimal(4, 2) not null,
    passes_attempted smallint not null,
    throws_attempted smallint not null,
    launch_to_passes_ratio decimal(4, 2) not null,
    avg_pass_length decimal(6, 2) not null,
    goal_kicks_attempted smallint not null,
    goal_kick_launch_ratio decimal(4, 2) not null,
    goal_kick_avg_length decimal(6, 2) not null,
    crosses_into_penalty_area_faced smallint not null,
    crosses_into_penalty_area_stopped smallint not null,
    crosses_into_penalty_area_stop_ratio decimal(4, 2) not null,
    defensive_actions_outside_penalty_area smallint not null,
    avg_distance_from_goal_per_defensive_action decimal(6, 2) not null,
    ts_gk_name tsvector,


    constraint fk_fixture_id_is_not_valid 
	foreign key (fixture_id) references fixture_tbl(fixture_id),
    
    constraint fk_team_id_is_not_valid 
	foreign key (team_id) references team_tbl(team_id)
);


create function fill_gk_stats_tbl_player_name_text_search_column()
  returns trigger as
$$
begin
  new.ts_gk_name := to_tsvector(new.gk_name);
  return new;
end;    
$$ language plpgsql;

create trigger update_gk_stats_tokens
  before update or insert on gk_stats_tbl
  for each row execute procedure fill_gk_stats_tbl_player_name_text_search_column();


create 
    index ts_gk_name_idx on gk_stats_tbl 
    using gin (ts_gk_name);
