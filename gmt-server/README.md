# Gmt-server

The gmt-server crate is the actual ssh and git server used by Git Mentor. It connects to the database to identify the users, their permissions and allows or refuse the ssh request.

## Design choices

The heavy lifting for both git and ssh are managed in external crates. This one only defines the interactions with the database.