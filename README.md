[![Build Status](https://travis-ci.com/avitex/rust-aliasable.svg?branch=master)](https://travis-ci.com/avitex/rust-aliasable)
[![Coverage Status](https://codecov.io/gh/avitex/rust-aliasable/branch/master/graph/badge.svg?token=X2LXHI8VYL)](https://codecov.io/gh/avitex/rust-aliasable)
[![Crate](https://img.shields.io/crates/v/aliasable.svg)](https://crates.io/crates/aliasable)
[![Docs](https://docs.rs/aliasable/badge.svg)](https://docs.rs/aliasable)

# rust-aliasable

**Rust library providing basic aliasable (non `core::ptr::Unique`) types**  
Documentation hosted on [docs.rs](https://docs.rs/aliasable).

```toml
aliasable = "0.1"
```

## Usage

```rust
use aliasable::vec::AliasableVec;

// Rexported via `aliasable::vec::UniqueVec`
let unique = Vec::from(&[1, 2, 3][..]);
let aliasable = AliasableVec::from(unique);
```
