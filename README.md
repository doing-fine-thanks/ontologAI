# A Summary Server

This server produces summaries of web pages. Its a work in progress. 

The server code can be found in `src`, the templates for html response in `templates`, and the DB migrations in `migrations`.

## To Get Started

The only external dependancy is a postgresql server _somewhere_ on your machine. Docker is fine, but I will leave
port forwarding the since as an exercise to those reading. 

Oh, also you'll need the cli for [sqlx](https://crates.io/crates/sqlx-cli).

First, install the sqlx-cli and run the the migrations:
```bash
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
#...some install logs...
$ sqlx migrate run   
Applied 20230718200943/migrate init (15.00775ms)
```

Next, put your DB url in a reasonable place:
```bash 
$ export DATABASE_URL=postgres://<your>:<db>@<address>:<here>/datasent
```



Then do the following if you want debug messaging:
```bash
$ RUST_LOG=debug cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.32s
Running `target/debug/next-thing-plugin-server`
[2023-07-20T23:22:17Z INFO  actix_server::builder] starting 10 workers
[2023-07-20T23:22:17Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
```

Or if you don't want logs:
```bash
$ cargo run 
Finished dev [unoptimized + debuginfo] target(s) in 0.12s
Running `target/debug/next-thing-plugin-server`
```