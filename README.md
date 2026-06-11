# securechat
Mini-project for my Y12 Software Engingeering Major Work

# About
This is meant to be an end-to-end encrypted messaging app based off of Signal's X3DH Key Agreement Protocol.
https://signal.org/docs/specifications/x3dh/

## Pre-requisites
1. Install the latest version of Rust here
   https://rust-lang.org/tools/install/
   Follow any additional steps you find there in fully setting up Rust
   Some useful information:
   https://doc.rust-lang.org/book/ch01-01-installation.html
   https://users.rust-lang.org/t/install-rust-error/126530
2. Install postgres 18 for your platform
   https://www.postgresql.org/download/
3. Install the sqlx-cli
   cargo install sqlx-cli
## Setup-guide
1. Clone the repository to your favourite location
2. Ensure that your current working directory is inside the root of one of the workspaces (client/server)
3. For building server:
   1. move your current directory into `infra` and ensure that you have the `DATABASE_URL` environment variable properly set in a .env at the root of the `infra` crate
   2. Ensure that the database is running
   3. run `sqlx migrate run` in the terminal
   4. move your current directory back to the root of server
   5. run `cargo run --release` in the terminal 
5. build and run program (cargo run --release) for each workspace
6. Enjoy (if it works)

## important note
I never finished this project so it doesn't work
