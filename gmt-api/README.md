# GMT-API

This is the API for the GMT project. It is a RESTful API that allows users to interact with the GMT database. The API is written in Rust using the [poem](https://crates.io/crates/poem) framework, which allows it to export all the APIs as OpenAPI json spec files. These files are in turn used by the frontend to abstract the communication between the two projects.

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

## Contributing

### Creating a new API

If your API fits within an existing module, you can add it there. If it doesn't, you can create a new module. To do this, you need to create a new module in the `src/services/mod.rs` module. The module should create a struct representing the service and implement the routes with this syntax:

```rust
#[OpenApi]
impl HelloService {
  #[oai(path = "/route", method = "get")]
  async fn my_route(&self, token: GmtToken) -> Result<Json<MyResponseType>, MyErrorType> {
    // ...
  }
}
```

Note the token parameter, used to ensure a user is logged in. The `HelloService` provides a set of example routes.