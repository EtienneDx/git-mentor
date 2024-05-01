# GMT-API

This is the API for the GMT project. It is a RESTful API that allows users to interact with the GMT database. The API is written in Rust using the [poem](https://crates.io/crates/poem) framework, which allows it to export all the APIs as OpenAPI json spec files. These files are in turn used by the frontend to abstract the communication between the two projects.

## Running the project

To run the project, you need to have Rust installed. To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

You can launch the database and start the project by running:

```sh
docker-compose up -d
cargo run
```

The service environment variables are defined in the .env file. In production, these would need to be replaced.

## Running the tests

To run the tests, you can use the following command:

```sh
cargo test
```