CREATE TYPE CommentAuthor AS ENUM ('user', 'automated');

CREATE TABLE Comments (
  id SERIAL PRIMARY KEY,
  repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  commit_hash VARCHAR(255) NOT NULL,
  
  respond_to INTEGER REFERENCES comments(id) ON DELETE CASCADE,
  file_path VARCHAR(255) NULL,

  message TEXT NOT NULL,
  author_type CommentAuthor NOT NULL,
  author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
  date TIMESTAMP NOT NULL
);
