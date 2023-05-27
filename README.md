# litewallet

A full-stack application that simulates a simple bank application, written completely in Rust.
This is a side-project and it's code _should not be used in production_ due to it's security vulnerabilities.
The project is more of a proof-of-concept and an exercise to learn using Rust in a bigger scale.
There are lots of shortcuts that have been taken, the app is supposed to be as simple as possible.

## How to run

1. Server:
   `cargo run`
2. Client:
   `cargo make serve`

## Backend

The server is built using `rocket-rs`, using a minimal setup with `serde` and `diesel-rs` for data fetching from a SQLite database.

The server exposes endpoints to let users authenticate, create "accounts", generate sessions and transfer from one account to another.

### Auth and Sessions

Authentication is built using sessions, storing them in a separate table in the SQLite db.
Sessions are stored as cookies on the client. These _should be_ HTTP-only cookies with expiry time.
This strategy is far from perfect, in a production app storing the cookies as HTTP-only or in-memory to avoid malicious third-party scripts from accessing it. Introducing JWT tokens with refresh token strategy can be a better way of handling Authentication in the long run.

### Accounts

There are potential flaws with the accounts logic both related to how it was implemented and the limitations of SQLite.
SQLite (with diesel) only allows storing integers that are signed and 32bit. This doesn't allow us to store 11-digit long account numbers as integers in the database.
Same thing applies to account balance, floats and big numbers aren't meant to be stored so just for simplicity sake these are stored as TEXT type and parsed during API calls.
Using a more powerful database could be a better decision for a production faced app. However SQLite is just fine for small projects like this one.

## Client

### Structure

The client app uses `seed-rs` to render a reactive frontend web app, it follows an Elm-inspired architecture.
