CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  pubkey TEXT[] NOT NULL
);

CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  teacher_id SERIAL REFERENCES users(id)
);

CREATE TABLE group_students (
  group_id SERIAL REFERENCES groups(id),
  student_id SERIAL REFERENCES users(id),
  PRIMARY KEY (group_id, student_id)
);
