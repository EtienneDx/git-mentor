CREATE TYPE CommentType AS ENUM ('default', 'response', 'line');

CREATE TYPE CommentAuthor AS ENUM ('user', 'automated');

CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  repository_id SERIAL REFERENCES repositories(id) ON DELETE CASCADE,
  commit_hash VARCHAR(255) NOT NULL,
  comment_type CommentType NOT NULL,
  message TEXT NOT NULL,
  author_type CommentAuthor NOT NULL,
  author_id SERIAL REFERENCES users(id) ON DELETE SET NULL,
  date TIMESTAMP NOT NULL
);
