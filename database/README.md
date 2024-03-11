# Database crate

This crates manages the database access and the underlying schema. It is managed using diesel.

## Installing the CLI

To install the CLI, run the following command:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Starting the database

For reusability, a test database can be created using docker-compose:

```bash
docker-compose up -d
```

You can then stop it using:

```bash
docker-compose down
```

## Migrations

The migrations are run on each test run to ensure that the database is up to date.

It is recommended to create migrations manually to keep it in sync with the schema.

## Generating the schema

To generate the schema, run the following command:

```bash
diesel print-schema > src/schema.rs
```

Don't forget to save the file as utf-8 to avoid encoding issues.