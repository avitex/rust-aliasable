# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- `AliasableMut` is now `repr(transparent)` (thanks [@talchas]).
- Implement derivable traits (thanks [@Kestrer]).

### Fixed
- `AliasableMut` variance issue #3 (thanks [@talchas]).
- `AliasableVec` provenance issue #6 (thanks [@saethlin]).

### Added
- Expose `AliasableVec` parts with `len/capacity/as_ptr/as_ptr_mut/is_empty`.
- [`unsize::CoerciblePtr`](https://docs.rs/unsize/1.1.0/unsize/trait.CoerciblePtr.html)
  support for `AliasableBox` (thanks [@HeroicKatora]).

## [0.1.3] - 2020-01-10

### Added
- `prelude` module.
- `AliasableMut` (thanks [@Koxiaet]).

[@Koxiaet]: https://github.com/Koxiaet  
[@HeroicKatora]: https://github.com/HeroicKatora  
[@talchas]: https://github.com/talchas
[@Kestrer]: https://github.com/Kestrer
[@saethlin]: https://github.com/saethlin
