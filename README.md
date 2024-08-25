# Outputter

Given a command to run, `Outputter` will display its output stream (`stdout` and `stderr`) in an interractive TUI.

## Demo
1. This project is package with Nix Flakes. You can use `nix develop` to drop into a shell with all the necessary dependency (for now, just the Rust toolchain).
```shell
$ nix develop
```

2. Build the demo app that output one line of log every seconds, alternating between `stdout` and `stderr`
```shell
$ cargo build --bin alternate
```
It will generate a binary at `target/debug/alternate`.

3. Launch `Outputter` on the demo app.
You should see one line added every second.
```shell
$ cargo run -- target/debug/alternate
```
Press `q` to quit.
