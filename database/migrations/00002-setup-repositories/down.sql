DROP TABLE CiRun;
DROP TYPE Status;

ALTER TABLE repositories DROP CONSTRAINT assignment_id_fk;
DROP TABLE assignments;

DROP TABLE repositories;

DROP TYPE RepoType;