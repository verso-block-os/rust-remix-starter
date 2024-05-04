create table todos (
  id serial primary key,
  title text not null,
  completed boolean not null default false
);

CREATE TYPE user_role AS ENUM ('admin', 'author', 'reader');

create table users (
  id serial primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  role user_role not null default 'reader',
  email text not null UNIQUE,
  password text not null
);

create table sessions (
  id serial primary key,
  user_id integer not null references users(id),
  expires_at timestamptz not null,
  token text not null UNIQUE
);

create table posts (
  id serial primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  user_id integer not null references users(id),
  title text not null,
  body text not null
);

create table comments (
  id serial primary key,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  user_id integer not null references users(id),
  post_id integer not null references posts(id),
  body text not null
);