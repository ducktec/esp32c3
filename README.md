# esp32c3

[![crates.io](https://img.shields.io/crates/v/esp32c3.svg)](https://crates.io/crates/esp32c3)


A peripheral access crate for the [ESP32-C3 SoC](https://www.espressif.com/en/products/socs/esp32-c3). See the [`svd2rust repo`](https://github.com/rust-embedded/svd2rust) for more infomation on how to use this crate. Espressif provides an official [svd](https://github.com/espressif/svd) file for the ESP32-C3, which is integrated as a git submodule and used as the baseline for this repo.

_At this point in time, this baseline SVD file is patched using the [`svdtools`](https://github.com/stm32-rs/svdtools). The Espressif svd file is still in development, so in the future the additional patch step will likely no longer be necessary._

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!


## [`Documentation`](https://docs.rs/esp32c3)


## Building

Required dependencies:

- [form](https://crates.io/crates/form)
- [svd](https://github.com/stm32-rs/svdtools)
- [svd2rust](https://github.com/rust-embedded/svd2rust)
- xmllint

```
$ make
```

## Submitting new patches

The base svd file provided by Espressif **cannot** be edited. Instead the base svd file is manipulated through the svdtools patching tool while still necessary.

See [svd](https://github.com/stm32-rs/svdtools) and [stm32-rs yaml format](https://github.com/stm32-rs/stm32-rs#device-and-peripheral-yaml-format) for more infomation on the patching format.

Identified issues that make a patch necessary should also be reported in the [svd](https://github.com/espressif/svd) repository so that they can be adressed in the baseline file.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
