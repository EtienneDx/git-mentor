# Gmt-server

The gmt-server crate is the actual ssh and git server used by Git Mentor. It connects to the database to identify the users, their permissions and allows or refuse the ssh request.

## Design choices

The heavy lifting for both git and ssh are managed in external crates. This one only defines the interactions with the database.

## Running the project

To run the project, you need to have Rust installed. To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

You can start the project by running:

```sh
cargo run
```

The service environment variables are defined in the .env file. In production, these would need to be replaced.

## Running the tests

To run the tests, you can use the following command:

```sh
cargo test
```