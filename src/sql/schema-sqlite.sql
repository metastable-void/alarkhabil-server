-- vim: sw=2, ts=2, expandtab

PRAGMA encoding = 'UTF-8'; 
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS `author` (
  id INTEGER PRIMARY KEY,
  uuid BLOB UNIQUE NOT NULL,
  name BLOB NOT NULL DEFAULT '',
  registered_date INTEGER NOT NULL, -- seconds since UNIX epoch
  is_deleted INTEGER NOT NULL DEFAULT 0,
  description_text BLOB NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_author_uuid` ON `author` (
  uuid
);

CREATE TABLE IF NOT EXISTS `author_public_key` (
  id INTEGER PRIMARY KEY,
  author_id INTEGER NOT NULL,
  type BLOB NOT NULL,
  public_key BLOB UNIQUE NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_author_public_key_public_key` ON `author_public_key` (
  public_key
);

CREATE INDEX IF NOT EXISTS `index_author_public_key_author_id` ON `author_public_key` (
  author_id
);

CREATE TABLE IF NOT EXISTS `channel` (
  id INTEGER PRIMARY KEY,
  uuid BLOB UNIQUE NOT NULL,
  handle BLOB UNIQUE NOT NULL,
  name BLOB NOT NULL DEFAULT '',
  created_date INTEGER NOT NULL, -- seconds since UNIX epoch
  is_deleted INTEGER NOT NULL DEFAULT 0,
  description_text BLOB NOT NULL DEFAULT '',
  language_code BLOB NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_channel_uuid` ON `channel` (
  uuid
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_channel_handle` ON `channel` (
  handle
);

CREATE TABLE IF NOT EXISTS `channel_author` (
  id INTEGER PRIMARY KEY,
  channel_id INTEGER NOT NULL,
  author_id INTEGER NOT NULL,
  FOREIGN KEY(channel_id) REFERENCES channel(id) ON DELETE CASCADE,
  FOREIGN KEY(author_id) REFERENCES author(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS `index_channel_author_channel_id` ON `channel_author` (
  channel_id
);

CREATE INDEX IF NOT EXISTS `index_channel_author_author_id` ON `channel_author` (
  author_id
);

CREATE TABLE IF NOT EXISTS `post` (
  id INTEGER PRIMARY KEY,
  uuid BLOB UNIQUE NOT NULL,
  channel_id INTEGER NOT NULL,
  is_deleted INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY(channel_id) REFERENCES channel(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_post_uuid` ON `post` (
  uuid
);

CREATE INDEX IF NOT EXISTS `index_post_channel_id` ON `post` (
  channel_id
);

CREATE TABLE IF NOT EXISTS `revision` (
  id INTEGER PRIMARY KEY,
  uuid BLOB UNIQUE NOT NULL,
  post_id INTEGER NOT NULL,
  author_id INTEGER NOT NULL,
  created_date INTEGER NOT NULL, -- seconds since UNIX epoch
  is_deleted INTEGER NOT NULL DEFAULT 0,
  title BLOB NOT NULL,
  revision_text BLOB NOT NULL,
  FOREIGN KEY(post_id) REFERENCES post(id) ON DELETE CASCADE,
  FOREIGN KEY(author_id) REFERENCES author(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_revision_uuid` ON `revision` (
  uuid
);

CREATE INDEX IF NOT EXISTS `index_revision_post_id` ON `revision` (
  post_id
);

CREATE INDEX IF NOT EXISTS `index_revision_author_id` ON `revision` (
  author_id
);

CREATE TABLE IF NOT EXISTS `post_tag` (
  id INTEGER PRIMARY KEY,
  post_id INTEGER NOT NULL,
  name BLOB NOT NULL,
  FOREIGN KEY(post_id) REFERENCES post(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS `index_post_tag_post_id` ON `post_tag` (
  post_id
);

CREATE INDEX IF NOT EXISTS `index_post_tag_name` ON `post_tag` (
  name
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_post_tag` ON `post_tag` (
  id,
  name
);

CREATE TABLE IF NOT EXISTS `meta_page` (
  id INTEGER PRIMARY KEY,
  page_name BLOB UNIQUE NOT NULL,
  title BLOB NOT NULL,
  updated_date INTEGER NOT NULL, -- seconds since UNIX epoch
  page_text BLOB NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX IF NOT EXISTS `index_meta_page_page_name` ON `meta_page` (
  page_name
);
