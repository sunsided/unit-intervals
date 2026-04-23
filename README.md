# unit-interval

[![CI](https://github.com/sunsided/unit-interval/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/unit-interval/actions/workflows/ci.yml)
[![license: EUPL-1.2](https://img.shields.io/badge/license-EUPL--1.2-blue.svg)](https://github.com/sunsided/unit-interval/blob/main/Cargo.toml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![no_std compatible](https://img.shields.io/badge/no__std-compatible-informational.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

Small constrained float types for values in the closed intervals `[0, 1]` and
`[-1, 1]`.

`unit-interval` provides `UnitInterval` and `SignedUnitInterval` wrappers for
normalized `f32` and `f64` values. Constructors reject out-of-range values and
`NaN`, while saturating constructors clamp inputs into the nearest valid value.
Operations that may leave the interval are available in checked and saturating
forms; operations that are closed over the interval return constrained values
directly.

## Rationale

Many domains use floats whose valid range is smaller than the full floating-point
space:

- Probabilities and confidence scores live in `[0, 1]`.
- Interpolation factors, progress values, weights, blend factors, and opacity
  often live in `[0, 1]`.
- RGBA color channels are commonly represented as normalized channel values in
  `[0, 1]`.
- Sine and cosine results, joystick axes, audio pan, centered offsets, and
  balance controls live in `[-1, 1]`.

Encoding those ranges in the type system keeps checks close to input boundaries
and lets later code state its requirements directly. A function that accepts
`UnitInterval` does not need to rediscover whether `1.2`, `-0.1`, or `NaN` can
arrive.

## Examples

```rust
use unit_interval::UnitInterval;

let probability = UnitInterval::new(0.8).unwrap();
let clamped = UnitInterval::saturating(1.2);

assert_eq!(probability.get(), 0.8);
assert_eq!(clamped, UnitInterval::ONE);
assert_eq!(UnitInterval::<f32>::new(f32::NAN), None);
```

```rust
use unit_interval::{SignedUnitInterval, UnitInterval};

let axis = SignedUnitInterval::new(-0.5).unwrap();
let weight = UnitInterval::new(0.25).unwrap();

assert_eq!((axis * weight).get(), -0.125);
assert_eq!(axis.saturating_add(weight).get(), -0.25);
```

```rust
use unit_interval::UnitInterval;

fn mix(start: f32, end: f32, amount: UnitInterval) -> f32 {
    amount.lerp(start, end)
}

assert_eq!(mix(10.0, 20.0, UnitInterval::HALF), 15.0);
```

## Feature Flags

- `std` is enabled by default and provides APIs that require the Rust standard
  library. Disable default features for `no_std` use.
- `serde` enables transparent serialization and checked deserialization through
  the inner floating-point value.
- `rkyv` enables zero-copy serialization and checked deserialization through the
  inner floating-point value.
- `assertions` enables internal invariant assertions in non-test builds.
- `unsafe` enables unchecked constructors and operations such as
  `UnitInterval::new_unchecked` and `SignedUnitInterval::new_unchecked`.

Unsafe code is forbidden unless the `unsafe` feature is explicitly enabled. The
unchecked APIs are behind that feature gate and require the caller to prove the
value is inside the relevant interval and is not `NaN`.

## Development

This repository uses the Taskfile as the local automation entry point:

```sh
task fmt:check
task lint:check
task test:all-features
```
