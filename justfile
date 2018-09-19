fmt:
    cargo +nightly fmt -v --all

server:
    RUST_LOG=witnet=trace cargo run server
