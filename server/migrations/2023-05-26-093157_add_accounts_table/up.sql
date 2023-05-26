create table accounts (
  id integer primary key autoincrement not null,
  owner_id integer not null,
  account_number text not null,
  balance text not null default('0'),
  active integer not null default(1)
)
