create table users (
  user_id      uuid          not null default uuid_generate_v4(),
  email        email         not null,
  name         nonempty_text not null,
  password     nonempty_text,
  primary key (user_id),
  unique (email)
);

select util.add_timestamps('users');
