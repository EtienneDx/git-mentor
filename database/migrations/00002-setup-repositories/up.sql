CREATE TYPE RepoType AS ENUM ('default', 'ci');

CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  repo_type RepoType NOT NULL,
  owner_id SERIAL REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE assignments (
  id SERIAL PRIMARY KEY,
  group_id SERIAL REFERENCES groups(id),
  base_repo_id SERIAL NOT NULL REFERENCES repositories(id) ON DELETE RESTRICT,
  test_repo_id SERIAL REFERENCES repositories(id) ON DELETE SET NULL,
  correction_repo_id SERIAL REFERENCES repositories(id) ON DELETE SET NULL
);

CREATE TYPE Status AS ENUM ('success', 'pending', 'cancelled', 'failed');

CREATE TABLE CiRun (
  id SERIAL PRIMARY KEY,
  repository_id SERIAL NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  commit TEXT NOT NULL,
  status STATUS NOT NULL
);

