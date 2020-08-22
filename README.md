# NGAU: Non-Generic Angle Units
Minimal, non-generic newtype wrappers for the angular operations provided by
the Rust standard library.

The purpose of this library is to provide some type safety for angle units
compared to using plain `f32` and `f64`.

## Features
- Zero dependecies.
- No generics.
- No macros.
- Optional Serde support.

The functions provided by this libarary are intended to be used instead of
the standard library `f32::sin`, `f32::cos` etc. that return naked `f32`
values. Instead, the functions with the same names in this library return
either a `Rad` or `Deg`, eg. `Rad::sin` return a `Rad`, unlike the `f32::sin`
in the standard library that return a naked `f32` radian value.

It was created to fill in some holes while migrating a project from `cgmath`
to `glam`.

Roadmap
-------
- Add another module for `f64` angle types (?).
- Benchmark and experiment with `#[inline]`.

## License
[license]: #license

This repository is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution Licensing
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.