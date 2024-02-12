# OntologAI

This is a simple web server to inspect some of the sense-making of LLMs through non-textual visualizations.

## Getting Started

Firstly export your `OPENAI_API_KEY` to your environment:
```bash
$ export OPENAI_API_KEY=<you-key-here>
```

All that is left after that is run the server:
```rust
$ RUST_LOG=debug cargo run

   Compiling ontologai v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.84s
     Running `target/debug/ontologai`
[2024-02-12T07:14:18Z INFO  next_thing_plugin_server]  🌐 will server on: http://127.0.0.1:8080 🌐 
[2024-02-12T07:14:18Z INFO  actix_server::builder] starting 10 workers
[2024-02-12T07:14:18Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime

```