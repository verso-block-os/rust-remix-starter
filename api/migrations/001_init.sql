create table todos (
  id serial primary key,
  title text not null,
  completed boolean not null default false
);

create table users (
  id serial primary key,
  email text not null UNIQUE,
  password text not null
);

create table sessions (
  id serial primary key,
  user_id integer not null references users(id),
  expires_at timestamptz not null,
  token text not null UNIQUE
);