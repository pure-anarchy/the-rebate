create table shot_stats_tbl (
    fixture_id int not null,

    player_name varchar(80),
    team_id smallint not null,
    shot_outcome varchar(30) not null,
    distance_from_goal smallint not null,
    body_part varchar(60) not null,
    first_shot_creator varchar(80) not null,
    first_shot_creating_event varchar(50) not null,
    second_shot_creator varchar(80) not null,
    second_shot_creating_event varchar(50) not null,
    ts_player_name tsvector

);


create function fill_shot_stats_player_name_text_search_column()
  returns trigger as
$$
begin
  new.ts_player_name := to_tsvector(new.player_name);
  return new;
end;    
$$ language plpgsql;

create trigger update_shot_stat_player_name_tokens
  before update or insert on shot_stats_tbl
  for each row execute procedure fill_shot_stats_player_name_text_search_column();


create 
    index ts_shot_stats_player_name_idx 
    on shot_stats_tbl using gin (ts_player_name);
