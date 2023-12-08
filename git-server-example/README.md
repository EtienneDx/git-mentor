# Simple example

This project aims to provide a simple example of the use of the [git-server](../git-server/) project.

## Usage

You can run the example simply using:

```bash
cargo run
```

## Behavior

This project makes the repositories in the `repositories` directory available through the ssh protocol over port 2222.

Given a `test` repository, you can then clone it using:

```bash
git clone git://localhost:2222/test
```	

The endpoint behaves like any endpoint. You can push to it, pull from it, etc.

## Configuration

Currently, this example project provides no configuration, it just runs (that's a good starting point, right?).