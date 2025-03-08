-- Add migration script here
create table users (
  id varchar(24) not null,
  username varchar(255) not null,
  pass_hash varchar(97) not null,
  primary key (id)
);

create table boards (
  id bigint not null auto_increment,
  name varchar(255) not null,
  slug varchar(255) not null,
  primary key (id)
);

create table images (
  id varchar(24) not null,
  content longblob not null,
  mime_type varchar(255) not null,
  primary key (id)
);

create table posts (
  id bigint not null auto_increment,
  board_id bigint not null,
  parent_id bigint,
  user_id varchar(24),
  content text not null,
  image_id varchar(24),
  created_at bigint not null,
  primary key (id),
  foreign key (board_id) references boards(id),
  foreign key (user_id) references users(id),
  foreign key (parent_id) references posts(id),
  foreign key (image_id) references images(id)
);
