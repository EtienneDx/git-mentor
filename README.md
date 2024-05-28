![React workflow](https://github.com/EtienneDx/git-mentor/actions/workflows/react-unit-test.yml/badge.svg)
![Rust workflow](https://github.com/EtienneDx/git-mentor/actions/workflows/rust-unit-test.yml/badge.svg)
[![codecov](https://codecov.io/gh/EtienneDx/git-mentor/graph/badge.svg?token=HM8WJ7Q2RU)](https://codecov.io/gh/EtienneDx/git-mentor)

# Git mentor

This project holds the components to the [Git mentor](./docs/Git%20mentor.md) project.

## Components

- [database](./database/): The shared database for the different git mentor applications
- [docs](./docs/): Documentation of the project
- [git-server](./git-server/): A simple git server library
- [git-server-example](./git-server-example/): A simple example of the use of the git-server library
- [gmt-api](./gmt-api/): The APIs of the project
- [gmt-cdk](./gmt-cdk/): The CDK stacks of the project
- [gmt-common](./gmt-common/): The common components of the project, reused by the different applications
- [gmt-server](./gmt-server/): The git server application
- [gmt-web-app](./gmt-web-app/): The frontend application of the project
- [ssh-server](./ssh-server/): A simple ssh server library, reused by the git-server library

## Setting up the development environment

### Prerequisites

#### Rust

The project uses Rust. To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

#### Node.js

The frontend uses Node.js. To install Node.js, follow the instructions at [https://nodejs.org/en/download/](https://nodejs.org/en/download/). As of the 25/04/2024, the project uses v20.10

#### AWS CDK

To generate the resources used by the project in AWS, you need to install the cdk:

```sh
npm install -g aws-cdk
```

#### Database

To run the database, it is recommended to use docker and the provided docker-compose scripts. To install docker, follow the instructions at [https://docs.docker.com/get-docker/](https://docs.docker.com/get-docker/).

Otherwise, you would need to modify the environments of the project consuming the database. The database is a [Postgre-SQL](https://www.postgresql.org/) system.

### Running the project

1. Clone the repository:

```sh
git clone https://github.com/EtienneDx/git-mentor
```

2. Follow the instructions in each package.

Most backend (rust) projects can be runned directly using cargo:

```sh
cargo run
```

For the frontend, please refer to the [gmt-web-app README](./gmt-web-app/).

## Contributing

When a pull request is submitted, the project will run the tests, linters and vulnerability checks.

Provided a maintainer's approval, the proposed change will also be deployed on an AWS environment, allowing for manual testing. Please remember to turn off the instance after testing for pending PRs, as my wallet will thank you.

## Re-usable crates

Some rust crates defined here could someday be re-used and deployed in the crates.io repository. If you are interested in using one of the crates, please open an issue to discuss the possibility of deploying it.

The relevant crates here are:

- [git-server](./git-server/): A simple git server library
- [ssh-server](./ssh-server/): A simple ssh server library, reused by the git-server library

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
