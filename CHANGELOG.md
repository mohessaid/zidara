# Changelog

All notable changes to Zidara are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Rust workspace and the gauntlet: `make gauntlet` runs lint, tests, acceptance stories,
  wasm32 tests, Android and iOS checks, 100% line, function and branch coverage, and mutation
  testing. The same steps run in CI.
- `Score`: scores as integers in basis points, with fractions and means rounded half up.
- Design spec, M0–M1 implementation plan, ground rules (`AGENTS.md`), decision records and
  the project journal.
