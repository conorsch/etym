# syntax chck and linters
check:
  cargo check

alias lint := check

# build project
build:
  cargo -q build
  nix build --no-link

# fetch fresh copies of EtymOnline flight payloads, as fixed state for tests
# (HTML pages are Cloudflare-challenged; the RSC header gets the raw payload)
update-fixtures:
  curl -H 'RSC: 1' "https://www.etymonline.com/search?q=viking" > tests/fixture-viking.rsc
  curl -H 'RSC: 1' "https://www.etymonline.com/search?q=scrimshaw" > tests/fixture-scrimshaw.rsc

# run unit tests
test:
  cargo test

# run integration tests (requires network)
integration:
  cargo test --test integration --features integration

# bump an alpha release
bump:
  cargo release version alpha --execute
