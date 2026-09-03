# syntax chck and linters
check:
  cargo check

alias lint := check

# build project
build:
  cargo build
  nix build --no-link

# fetch fresh copies of EtymOnline pags, as fixed state for tests
update-fixtures:
  curl "https://www.etymonline.com/search?q=viking" | tidy > tests/fixture-viking.html || true
  curl "https://www.etymonline.com/search?q=scrimshaw" | tidy > tests/fixture-scrimshaw.html || true

# run unit tests
test:
  cargo test

# run integration tests (requires network)
integration:
  cargo test --test integration --features integration

# bump an alpha release
bump:
  cargo release version alpha --execute
