CREATE TABLE repositories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    repo_type RepoType NOT NULL,
    owner_id SERIAL REFERENCES users(id)
);

CREATE TABLE assignments (
    id SERIAL PRIMARY KEY,
    group_id SERIAL REFERENCES groups(id),
    base_repo_id SERIAL REFERENCES repositories(id),
    test_repo_id SERIAL REFERENCES repositories(id),
    correction_repo_id SERIAL REFERENCES repositories(id)
);