create table season_tbl (
    season_id int generated always as identity,
    season_name varchar(11) not null,
    league_id smallint not null,

    constraint fk_league_id_is_not_valid
	foreign key (league_id) references league_tbl (league_id),

    constraint season_name_is_not_unique
	unique(season_name),

    primary key(season_id)
);
