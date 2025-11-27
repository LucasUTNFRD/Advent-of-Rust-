lint:
    cargo fmt -- `find . -name "*.rs"`
    cargo clippy --all-targets --all-features

run:
    cargo run

run-release:
    cargo build --release  && ./target/release/aoc-rs
