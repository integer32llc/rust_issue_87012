# Rust Issue 87012 Test Case

This is a test case for <https://github.com/rust-lang/rust/issues/87012>.


## Usage
Follow <https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html> to install `summary` and
`crox`. You'll also need a nightly build.

Once the prep work is done, just run `./test.sh` and you'll get a the affected line (`evaluate_obligation`) from the
summary and a Chrome-compatible profile.

For reference: with `rustc 1.57.0-nightly (ac2d9fc50 2021-09-21)` the `evaluate_obligation` took around 30% of the time.


## Construction
There are two crates:

- `helper_crate`: This contains a bunch of async code that involves nested types and calls, like you could find in a
  larger application. The entire code has a single entrypoint.
- `repro_crate`: The actual reproducer that just uses a single entrypoint from `helper_crate` and uses it multiple times
  in an async-trait.


## Observation
It seems that some trait evaluation work is not done while compiling `helper_crate` (e.g. the `Send` check that is
required by async-trait because it marks the future as `Send`) and then must be done multiple times when compling
`repro_crate`.
