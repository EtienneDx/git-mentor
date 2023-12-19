CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    teacher_id SERIAL REFERENCES users(id)
);
