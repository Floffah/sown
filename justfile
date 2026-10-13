default:
    @just --list

build:
    cargo build --workspace --release

[arg("profile", long)]
check profile="test": clippy test (fixtures profile)
    cargo fmt --all --check

fixtures profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p cli -- check fixtures/minimal-room

alias run := run-client
run-client:
    cargo run --bin client

run-server:
    cargo run crates/server --world=fixtures/minimal-room

clippy:
    cargo clippy --locked --workspace --all-targets -- -D warnings -W clippy::pedantic

release profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p brixin-cli -- build fixtures/minimal-room

release-digest profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p brixin-cli -- build fixtures/minimal-room | cut -d' ' -f2

format:
    cargo fmt --all

[arg("profile", long)]
test profile="test":
    cargo test --locked --workspace --profile {{profile}}

doc:
    RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --document-private-items --all-features --lib --bins --examples --workspace --all --release
