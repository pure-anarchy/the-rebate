create table fixture_tbl (
    fixture_id int generated always as identity,
    season_id int not null,
    matchweek smallint not null,
    home_team_id int not null,
    away_team_id int not null,
    fixture_date date,
    fixture_time time,
    attendance int,

    primary key(fixture_id)
);


