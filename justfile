default:
    @just --list

build:
    cargo build --workspace --release

[arg("profile", long)]
check profile="test": (fixtures profile) clippy
    cargo fmt --all --check
    cargo test --locked --workspace --profile {{profile}}

fixtures profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p brixin-cli -- check fixtures/minimal-room

clippy:
    cargo clippy --locked --workspace --all-targets -- -D warnings -W clippy::pedantic

release profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p brixin-cli -- build fixtures/minimal-room

release-digest profile="dev":
    cargo run --locked --quiet --profile {{profile}} -p brixin-cli -- build fixtures/minimal-room | cut -d' ' -f2

format:
    cargo fmt --all

test:
    cargo test --locked --workspace

doc:
    RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --document-private-items --all-features --lib --bins --examples --workspace --all --release
