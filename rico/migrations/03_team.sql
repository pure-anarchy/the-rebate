create table team_tbl (
    team_id int generated always as identity,
    league_id smallint not null,
    team_name varchar(75) not null,

    constraint fk_league_id_is_not_valid 
	foreign key (league_id)  references league_tbl(league_id),
    
    constraint team_name_is_not_valid
	unique(team_name),


    primary key(team_id)
);



