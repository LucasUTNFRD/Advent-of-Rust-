lint:
    cargo fmt -- `find . -name "*.rs"`
    cargo clippy --all-targets --all-features

run:
    cargo run

run-release:
    cargo run --release


build-release:
    cargo build --release
