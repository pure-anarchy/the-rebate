create type match_record as (
    wins smallint,
    draws smallint,
    loses smallint
);



create table fixture_report_tbl (
    fixture_id int primary key,
    home_goals smallint not null,
    away_goals smallint not null, 
    home_xg decimal(6, 2) not null,
    away_xg decimal(6, 2) not null,
    home_match_record match_record not null,
    away_match_record match_record not null,
    home_possession decimal(4, 2) not null,
    away_possession decimal(4, 2) not null,
    home_passes smallint not null,
    away_passes smallint not null,
    home_accurate_passes smallint not null,
    away_accurate_passes smallint not null,
    home_shots smallint not null,
    away_shots smallint not null,
    home_shots_on_target smallint not null,
    away_shots_on_target smallint not null,
    home_gk_saves smallint not null,
    away_gk_saves smallint not null,
    home_yellow_cards smallint not null,
    away_yellow_cards smallint not null,
    home_fouls smallint not null,
    home_corners smallint not null,
    home_touches smallint not null,
    home_tackles smallint not null,
    home_interceptions smallint not null,
    home_aerials_won smallint not null,
    home_clearances smallint not null,
    home_offsides smallint not null,
    home_goal_kicks smallint not null,
    home_throw_ins smallint not null,
    home_long_balls smallint not null,
    away_fouls smallint not null,
    away_corners smallint not null,
    away_touches smallint not null,
    away_tackles smallint not null,
    away_interceptions smallint not null,
    away_aerials_won smallint not null,
    away_clearances smallint not null,
    away_offsides smallint not null,
    away_goal_kicks smallint not null,
    away_throw_ins smallint not null,
    away_long_balls smallint not null,

    
    constraint fk_fixture_id_is_not_valid
	foreign key (fixture_id) references fixture_tbl(fixture_id)

);




