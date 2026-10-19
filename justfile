mod site

default:
    @just --list

build: generate-protocol site::build
    cargo build --workspace --release

generate-protocol:
    buf generate
    cargo fmt -p protocol

[arg("profile", long)]
check profile="test": clippy test (fixtures profile)
    cargo fmt --all --check

fixtures profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p cli -- check fixtures/minimal-room

alias run := run-client
run-client:
    cargo run --bin client

[env("SSLKEYLOGFILE", justfile_directory() + "/.sown/tls-keys.log")]
run-server:
    cargo run --bin server

clippy:
    cargo clippy --locked --workspace --all-targets -- -D warnings -W clippy::pedantic

format:
    cargo fmt --all

[arg("profile", long)]
test profile="test":
    cargo test --locked --workspace --profile {{profile}}

doc:
    RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --document-private-items --all-features --lib --bins --examples --workspace --all --release
