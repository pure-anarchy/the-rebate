create table league_tbl (
    league_id smallint generated always as identity,
    league_name varchar(50) not null,

    constraint league_name_is_not_unique unique(league_name),
    primary key(league_id)
);
