# etym changelog

## Unreleased

* fix: query EtymOnline's RSC endpoint; HTML pages are now Cloudflare-challenged
* Refresh test fixtures as raw React flight payloads (`.rsc`)
* test: integration tests to exercise CLI interface
* Update nixpkgs to 26.05
* Add envrc flake for direnv integration
* Add justfile for common commands
* Cache nix build dependencies independently of the crate version

## 0.0.9 (2025-12-23)

* Add nix flake for easier installation

## 0.0.8 (2025-10-06)

* Fix HTML parsing for 2025 website

## 0.0.7 (2023-03-10)

* Better error handling, fewer unwraps
* No longer defaults to musl for build

## 0.0.6 (2022-12-10)

* Reorganize module as library
* Default to musl (statically linked) build
* Update dependencies

## 0.0.5 (2022-07-06)

* Rewrites project Python -> Rust

## 0.0.4 (2020-07-25)

* Basic CLI functionality
