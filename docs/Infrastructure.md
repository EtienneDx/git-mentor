# Infrastructure

## Overview

The project can be split into multiple components. First comes the main server, which will hold the git repositories. For the first iterations, the main server will **NOT** be parallelized.

Here is a simplified diagram of the project's architecture:

![Architecture](./graphs/infrastructure.svg)

As displayed here, the first iteration will bundle all components into a single server. This will allow for a simpler development and deployment. However, this adds restrictions on the scalability of the project. Later iterations will separate components over different services.

## Design principles

As per best practices and to favor a proper unit testing of the code, it is recommended to design everything with the dependency injection design principle in mind. This will make the code easier to iterate over, as well as provide easily testable indepenedant components.

## Components

### Frontend

The frontend will be a simple web application, allowing users to view their repositories and their content. It will also allow them to manage their repositories and their permissions. The frontend will be written in [React](https://reactjs.org/), and hosted on an [S3](https://aws.amazon.com/s3/) bucket, served by [CloudFront](https://aws.amazon.com/cloudfront/).

### CI queue

The CI queue will be a simple [SQS](https://aws.amazon.com/sqs/) queue. It will hold the CI jobs to be executed. The CI queue will be polled by the test workers.

### Main Server

The main server will be hosted on an [EC2](https://aws.amazon.com/ec2/) instance. It will host multiple applications:

#### SSH entrypoint

The SSH entrypoint will be the only way to access the git repositories. It will be a simple SSH server, with a public key authentication. It will be written in [Rust](https://www.rust-lang.org/).

The SSH and git server will be built into separate projects, to allow reusability of the code for any eventual future projects. The specifics of the git mentor server will be implemented in a third project depending on the two others.

The algorithm for the server is relatively simple:

![SSH server workflow](./graphs/ssh_process.svg)

#### HTTP entrypoint

The server will offer multiple REST APIs to allow the frontend to interact with the git repositories. It will be written in [Rust](https://www.rust-lang.org/).

The specifics of the HTTP server haven't been decided yet, between the use of an existing framework or the development of a custom one.

#### Git repositories

The git repositories will be bare repositories, stored on the server's filesystem. Both the SSH and HTTP entrypoints will interact with the repositories through the filesystem.

Git repositories will need to be initialized with some hooks installed, to allow the CI pipeline to be executed. Here is a diagram of the different git workflows:

![Git workflows](./graphs/git_process.svg)

#### Test worker

The test worker will be a simple application monitoring the CI queue. It will poll the queue for new jobs, and execute them. It will be written in [Rust](https://www.rust-lang.org/).

Here is an overview of the test worker's workflow:

![Test worker workflow](./graphs/test_worker.svg)

The test worker aims to be a self-contained project, allowing an easy migration to a microservice architecture at a later point. Jobs need to run in a [Docker](https://www.docker.com/) container, to ensure a clean environment for each job.

#### Database

The database will be a simple [PostgreSQL](https://www.postgresql.org/) database. It will hold the users, repositories and permissions.

During the first iteration, the database will be self-contained in the main server. Later iterations will move the database to a separate service, such as [RDS](https://aws.amazon.com/rds/).