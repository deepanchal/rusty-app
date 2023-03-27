# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources 
check:
  cargo clippy --locked -- -D warnings

# Runs unit tests
test:
  cargo test --locked

# Format with rustfmt
format:
  cargo fmt
