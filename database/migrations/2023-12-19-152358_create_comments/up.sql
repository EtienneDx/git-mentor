CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    repository_id SERIAL REFERENCES repositories(id),
    commit_hash VARCHAR(255) NOT NULL,
    comment_type CommentType NOT NULL,
    message TEXT NOT NULL,
    author_type CommentAuthor NOT NULL,
    author_id SERIAL REFERENCES users(id),
    date TIMESTAMP NOT NULL
);
