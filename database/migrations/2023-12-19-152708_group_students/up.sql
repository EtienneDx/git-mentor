CREATE TABLE group_students (
    group_id SERIAL REFERENCES groups(id),
    student_id SERIAL REFERENCES users(id),
    PRIMARY KEY (group_id, student_id)
);
