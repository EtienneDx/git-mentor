CREATE TABLE CiRun (
    id SERIAL PRIMARY KEY,
    repository_id INT REFERENCES repositories(id),
    commit TEXT NOT NULL,
    status STATUS NOT NULL
);

