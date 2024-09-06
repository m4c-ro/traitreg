# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

-

### Added

-

### Changed

-

### Removed

-

## [0.3.0] - 2024-09-07

### Fixed

-

### Added

-

### Changed

- Add std::mutex::Mutex around internal ```__REGISTRY``` static
- TraitRegStorage initialization implemented internally, rather than in macro generated code

### Removed

- unsafe around static mut internally
- Default implementation on TraitRegStorage
- 'examples' and 'tests' directories from package

## [0.2.0] - 2024-09-06

### Added

- Better examples
- Tests for correct API usage
- Tests for incorrect API usage
- A test for initialization order

### Fixed

- Initialization order of registrations before registry

### Changed

- Replace references to std interally with references to core

## [0.1.0] - 2024-09-06

Initial unstable release
