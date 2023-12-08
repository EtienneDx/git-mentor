# Git-Server

## Description

This is a simple git server that can be used to host git repositories. It is only a library and does not provide any executable. There is an example project that uses this library [here](example).

## Usage

You can use this library by adding the following to your `Cargo.toml`:

```toml
[dependencies]
git-server = { git = "https://github.com/EtienneDx/git-server.git" }
```

To create a server, you first need objects implementing a couple of traits:

- `RepositoryProvider`: This trait is used to provide the repositories to the server. It is used to get a repository from a path given by a user.
- `Authenticator`: This trait is used to authenticate the users. It is used to identify a user given it's public key.
- `Repository`: This trait is used to represent a repository. It is used to validate whether a user has access to a repository or not.

You can use your own `User` type, which does not require any specific trait. See the [example](example) for an example of how to do that.

Once you have these objects, you can create a server using the `GitServer::new` method. You can then run the server using the `GitServer::listen` method.