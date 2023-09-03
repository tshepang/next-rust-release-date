# next-rust-release-date

[![crates.io](https://img.shields.io/crates/v/next-rust-release-date.svg)](https://crates.io/crates/next-rust-release-date)
[![build status](https://github.com/tshepang/next-rust-release-date/workflows/CI/badge.svg)](https://github.com/tshepang/next-rust-release-date/actions)

Answers "when is the next feature release of Rust?"

Following is the sort of output to expect:

```
  2018-09-13 - Rust 1.29
* 2018-10-25 - Rust 1.30
  2018-12-06 - Rust 1.31
```

## Installation

Assuming you have the [Rust toolchain installed][install]:

    cargo install next-rust-release-date

NOTE:
minimum required rustc is v1.60,
since earlier toolchains fail due to a dependency resolution fail.

[install]: https://rust-lang.org/install

#### License

<sup>
Licensed under either of
<a href="LICENSE-APACHE">Apache License, Version 2.0</a>
or
<a href="LICENSE-MIT">MIT license</a>
at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
