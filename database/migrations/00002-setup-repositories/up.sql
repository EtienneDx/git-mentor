CREATE TYPE RepoType AS ENUM ('default', 'ci');

CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  repo_type RepoType NOT NULL,
  owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
  assignment_id INTEGER
);

CREATE TABLE assignments (
  id SERIAL PRIMARY KEY,
  group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
  base_repo_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE RESTRICT,
  test_repo_id INTEGER REFERENCES repositories(id) ON DELETE SET NULL,
  correction_repo_id INTEGER REFERENCES repositories(id) ON DELETE SET NULL
);

ALTER TABLE repositories ADD CONSTRAINT assignment_id_fk FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE SET NULL;

CREATE TYPE Status AS ENUM ('success', 'pending', 'cancelled', 'failed');

CREATE TABLE CiRun (
  id SERIAL PRIMARY KEY,
  repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  commit TEXT NOT NULL,
  status STATUS NOT NULL
);

