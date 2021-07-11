## bono backend

The backend of bono. Licensed under agpl.

### Getting set up:
This is in very early stages of development and isn't reccomended for production yet.
 - Install the latest stable version of Rust
 - Install sqlite and sqlite3
 - Either:
    - Run `sqlite3 bono.db` in the root directory to create a blank database
    - Rename `bono-sample.db` to `bono.db` to use a development sample database
 - `cargo run` to compile and run the server (for development.)
 - <b>Note:</b> sqlx, our database library, interfaces with the db to typecheck. If you use a live code analyzer in your ide, make sure the `DATABASE_URL` env var is set when it runs.

 ### Design goals
  - Ease of setup. Ideally, have some sort of script to automate a process from download to server run.
  - Use few resources. This should be able to run on a Raspi or other low power arm device.