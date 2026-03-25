# build project; if argument "--release" is provided, the build is in the Release mode
build *ARGS:
    cargo build {{ARGS}}

# run cargo check
check:
    cargo check

# run cargo clippy
clippy:
    cargo clippy

# run cargo fmt
format:
    cargo fmt

# run backend server
serve-backend:
    cargo run -p audiora-me-server
