check:
  cargo check

build:
  cargo build

update-fixtures:
  curl "https://www.etymonline.com/search?q=viking" | tidy > tests/fixture-viking.html || true
  curl "https://www.etymonline.com/search?q=scrimshaw" | tidy > tests/fixture-scrimshaw.html || true

test:
  cargo test
