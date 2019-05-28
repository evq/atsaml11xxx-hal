# atsaml11xxx-hal

The very early beginnings of an `embedded-hal` implementation for the SAM L11 series.

It also includes a rand_core rng implementation that uses the hardware rng and code to use the rom built-in SHA256.

The [atsaml11xxx](https://github.com/evq/atsaml11xxx) crate is used for for peripheral access.

Most `embedded-hal` implementation code is taken from [atsamd](https://github.com/atsamd-rs/atsamd) with a few
hacks and tweaks for the L11.

# WIP

This crate is a work in progress.

# Tests

There are a few tests for the code supporting the rom built-in SHA256.
```
cargo test --target x86_64-unknown-linux-gnu --lib
```

# License

`src/rng.rs` and `src/crypto` - Copyright (c) 2019 eV Quirk - MIT License

`embedded-hal` implementation - [Derived from atsamd licensed under MIT / Apache 2.0](https://github.com/atsamd-rs/atsamd#license)
