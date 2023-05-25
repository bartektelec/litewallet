create table sessions (
  id integer unique primary key not null,
  user_id integer unique not null,
  session_id text unique
);
