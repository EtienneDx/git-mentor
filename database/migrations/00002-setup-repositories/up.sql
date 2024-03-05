CREATE TYPE RepoType AS ENUM ('Default', 'CI');

CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  repo_type RepoType NOT NULL,
  owner_id SERIAL REFERENCES users(id)
);

CREATE TABLE assignments (
  id SERIAL PRIMARY KEY,
  group_id SERIAL REFERENCES groups(id),
  base_repo_id SERIAL NOT NULL REFERENCES repositories(id),
  test_repo_id SERIAL REFERENCES repositories(id),
  correction_repo_id SERIAL REFERENCES repositories(id)
);

CREATE TYPE Status AS ENUM ('Success', 'Pending', 'Cancelled', 'Failed');

CREATE TABLE CiRun (
  id SERIAL PRIMARY KEY,
  repository_id SERIAL NOT NULL REFERENCES repositories(id),
  commit TEXT NOT NULL,
  status STATUS NOT NULL
);

