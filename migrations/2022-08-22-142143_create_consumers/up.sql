-- Your SQL goes here
CREATE TABLE consumers (
  id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
  tel_user_id varchar(50) NOT NULL,
  username varchar(255),
  first_name varchar(255) NOT NULL,
  last_name varchar(255),
  is_bot boolean NOT NULL DEFAULT FALSE,
  is_admin boolean NOT NULL DEFAULT FALSE,
  created_at datetime  NOT NULL DEFAULT current_timestamp,
  updated_at datetime  NOT NULL DEFAULT current_timestamp
);
