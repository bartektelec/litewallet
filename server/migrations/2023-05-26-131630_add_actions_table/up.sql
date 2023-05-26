create table actions (
  id integer primary key autoincrement not null,
  created_at text not null,
  from_acc text not null,
  to_acc text not null,
  amount text not null
)
