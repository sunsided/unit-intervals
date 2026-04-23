#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use unit_intervals::{SignedUnitInterval, UnitInterval};

#[derive(Debug, Arbitrary)]
struct Input {
    a32: u32,
    b32: u32,
    factor32: u32,
    a64: u64,
    b64: u64,
    factor64: u64,
}

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);

    if let Ok(input) = Input::arbitrary(&mut unstructured) {
        check_unit_f32(input.a32, input.b32, input.factor32);
        check_signed_f32(input.a32, input.b32, input.factor32);
        check_unit_f64(input.a64, input.b64, input.factor64);
        check_signed_f64(input.a64, input.b64, input.factor64);
    }

    check_arbitrary_feature(data);
});

fn check_unit_f32(a_bits: u32, b_bits: u32, factor_bits: u32) {
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    let factor = f32::from_bits(factor_bits);

    assert_eq!(
        UnitInterval::<f32>::new(a).is_some(),
        UnitInterval::<f32>::contains(a)
    );
    assert!(UnitInterval::<f32>::contains(
        UnitInterval::<f32>::saturating(a).get()
    ));

    if let Some(a) = UnitInterval::<f32>::new(a) {
        assert_unit_f32_invariants(a);

        if let Some(b) = UnitInterval::<f32>::new(b) {
            assert_unit_f32_invariants(b);
            assert_unit_f32_invariants(a * b);
            assert_unit_f32_invariants(a.complement());
            assert_unit_f32_invariants(a.min(b));
            assert_unit_f32_invariants(a.max(b));
            assert_unit_f32_invariants(a.midpoint(b));
            assert_unit_f32_invariants(a.distance_to(b));
            assert_checked_unit_f32(a.checked_add(b), a.get() + b.get());
            assert_checked_unit_f32(a.checked_sub(b), a.get() - b.get());
            assert_checked_unit_f32(a.checked_div(b), a.get() / b.get());
            assert_unit_f32_invariants(a.saturating_add(b));
            assert_unit_f32_invariants(a.saturating_sub(b));
            assert_unit_f32_invariants(a.saturating_div(b));
        }

        assert_checked_unit_f32(a.checked_scale(factor), a.get() * factor);
        assert_unit_f32_invariants(a.saturating_scale(factor));

        check_unit_f32_optional_features(a, factor);
    }
}

fn assert_unit_f32_invariants(value: UnitInterval<f32>) {
    let inner = value.get();

    assert!(UnitInterval::<f32>::contains(inner));
    assert!(!inner.is_nan());
}

fn assert_checked_unit_f32(value: Option<UnitInterval<f32>>, raw: f32) {
    assert_eq!(value.is_some(), UnitInterval::<f32>::contains(raw));

    if let Some(value) = value {
        assert_unit_f32_invariants(value);
    }
}

fn check_signed_f32(a_bits: u32, b_bits: u32, factor_bits: u32) {
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    let factor = f32::from_bits(factor_bits);

    assert_eq!(
        SignedUnitInterval::<f32>::new(a).is_some(),
        SignedUnitInterval::<f32>::contains(a)
    );
    assert!(SignedUnitInterval::<f32>::contains(
        SignedUnitInterval::<f32>::saturating(a).get()
    ));

    if let Some(a) = SignedUnitInterval::<f32>::new(a) {
        assert_signed_f32_invariants(a);
        assert_unit_f32_invariants(a.abs());
        assert_signed_f32_invariants(-a);
        assert_signed_f32_invariants(a.signum());
        assert_signed_f32_invariants(a.copysign(factor));

        if let Some(b) = SignedUnitInterval::<f32>::new(b) {
            assert_signed_f32_invariants(b);
            assert_signed_f32_invariants(a * b);
            assert_signed_f32_invariants(a.min(b));
            assert_signed_f32_invariants(a.max(b));
            assert_signed_f32_invariants(a.midpoint(b));
            assert_signed_distance_f32(a.distance_to(b), a.get(), b.get());
            assert_checked_signed_f32(a.checked_add(b), a.get() + b.get());
            assert_checked_signed_f32(a.checked_sub(b), a.get() - b.get());
            assert_checked_signed_f32(a.checked_div(b), a.get() / b.get());
            assert_signed_f32_invariants(a.saturating_add(b));
            assert_signed_f32_invariants(a.saturating_sub(b));
            assert_signed_f32_invariants(a.saturating_div(b));
        }

        assert_checked_signed_f32(a.checked_scale(factor), a.get() * factor);
        assert_signed_f32_invariants(a.saturating_scale(factor));

        check_signed_f32_optional_features(a, factor);
    }
}

fn assert_signed_f32_invariants(value: SignedUnitInterval<f32>) {
    let inner = value.get();

    assert!(SignedUnitInterval::<f32>::contains(inner));
    assert!(!inner.is_nan());
}

fn assert_checked_signed_f32(value: Option<SignedUnitInterval<f32>>, raw: f32) {
    assert_eq!(value.is_some(), SignedUnitInterval::<f32>::contains(raw));

    if let Some(value) = value {
        assert_signed_f32_invariants(value);
    }
}

fn assert_signed_distance_f32(distance: f32, a: f32, b: f32) {
    assert!(distance >= 0.0);
    assert!(distance <= 2.0);
    assert_eq!(distance, (a - b).abs());
}

fn check_unit_f64(a_bits: u64, b_bits: u64, factor_bits: u64) {
    let a = f64::from_bits(a_bits);
    let b = f64::from_bits(b_bits);
    let factor = f64::from_bits(factor_bits);

    assert_eq!(
        UnitInterval::<f64>::new(a).is_some(),
        UnitInterval::<f64>::contains(a)
    );
    assert!(UnitInterval::<f64>::contains(
        UnitInterval::<f64>::saturating(a).get()
    ));

    if let Some(a) = UnitInterval::<f64>::new(a) {
        assert_unit_f64_invariants(a);

        if let Some(b) = UnitInterval::<f64>::new(b) {
            assert_unit_f64_invariants(b);
            assert_unit_f64_invariants(a * b);
            assert_unit_f64_invariants(a.complement());
            assert_unit_f64_invariants(a.min(b));
            assert_unit_f64_invariants(a.max(b));
            assert_unit_f64_invariants(a.midpoint(b));
            assert_unit_f64_invariants(a.distance_to(b));
            assert_checked_unit_f64(a.checked_add(b), a.get() + b.get());
            assert_checked_unit_f64(a.checked_sub(b), a.get() - b.get());
            assert_checked_unit_f64(a.checked_div(b), a.get() / b.get());
            assert_unit_f64_invariants(a.saturating_add(b));
            assert_unit_f64_invariants(a.saturating_sub(b));
            assert_unit_f64_invariants(a.saturating_div(b));
        }

        assert_checked_unit_f64(a.checked_scale(factor), a.get() * factor);
        assert_unit_f64_invariants(a.saturating_scale(factor));
    }
}

fn assert_unit_f64_invariants(value: UnitInterval<f64>) {
    let inner = value.get();

    assert!(UnitInterval::<f64>::contains(inner));
    assert!(!inner.is_nan());
}

fn assert_checked_unit_f64(value: Option<UnitInterval<f64>>, raw: f64) {
    assert_eq!(value.is_some(), UnitInterval::<f64>::contains(raw));

    if let Some(value) = value {
        assert_unit_f64_invariants(value);
    }
}

fn check_signed_f64(a_bits: u64, b_bits: u64, factor_bits: u64) {
    let a = f64::from_bits(a_bits);
    let b = f64::from_bits(b_bits);
    let factor = f64::from_bits(factor_bits);

    assert_eq!(
        SignedUnitInterval::<f64>::new(a).is_some(),
        SignedUnitInterval::<f64>::contains(a)
    );
    assert!(SignedUnitInterval::<f64>::contains(
        SignedUnitInterval::<f64>::saturating(a).get()
    ));

    if let Some(a) = SignedUnitInterval::<f64>::new(a) {
        assert_signed_f64_invariants(a);
        assert_unit_f64_invariants(a.abs());
        assert_signed_f64_invariants(-a);
        assert_signed_f64_invariants(a.signum());
        assert_signed_f64_invariants(a.copysign(factor));

        if let Some(b) = SignedUnitInterval::<f64>::new(b) {
            assert_signed_f64_invariants(b);
            assert_signed_f64_invariants(a * b);
            assert_signed_f64_invariants(a.min(b));
            assert_signed_f64_invariants(a.max(b));
            assert_signed_f64_invariants(a.midpoint(b));
            assert_signed_distance_f64(a.distance_to(b), a.get(), b.get());
            assert_checked_signed_f64(a.checked_add(b), a.get() + b.get());
            assert_checked_signed_f64(a.checked_sub(b), a.get() - b.get());
            assert_checked_signed_f64(a.checked_div(b), a.get() / b.get());
            assert_signed_f64_invariants(a.saturating_add(b));
            assert_signed_f64_invariants(a.saturating_sub(b));
            assert_signed_f64_invariants(a.saturating_div(b));
        }

        assert_checked_signed_f64(a.checked_scale(factor), a.get() * factor);
        assert_signed_f64_invariants(a.saturating_scale(factor));
    }
}

fn assert_signed_f64_invariants(value: SignedUnitInterval<f64>) {
    let inner = value.get();

    assert!(SignedUnitInterval::<f64>::contains(inner));
    assert!(!inner.is_nan());
}

fn assert_checked_signed_f64(value: Option<SignedUnitInterval<f64>>, raw: f64) {
    assert_eq!(value.is_some(), SignedUnitInterval::<f64>::contains(raw));

    if let Some(value) = value {
        assert_signed_f64_invariants(value);
    }
}

fn assert_signed_distance_f64(distance: f64, a: f64, b: f64) {
    assert!(distance >= 0.0);
    assert!(distance <= 2.0);
    assert_eq!(distance, (a - b).abs());
}

#[cfg(feature = "arbitrary")]
fn check_arbitrary_feature(data: &[u8]) {
    use arbitrary::Arbitrary as _;

    let mut input = Unstructured::new(data);

    if let Ok(value) = UnitInterval::<f32>::arbitrary(&mut input) {
        assert_unit_f32_invariants(value);
    }

    if let Ok(value) = UnitInterval::<f64>::arbitrary(&mut input) {
        assert_unit_f64_invariants(value);
    }

    if let Ok(value) = SignedUnitInterval::<f32>::arbitrary(&mut input) {
        assert_signed_f32_invariants(value);
    }

    if let Ok(value) = SignedUnitInterval::<f64>::arbitrary(&mut input) {
        assert_signed_f64_invariants(value);
    }
}

#[cfg(not(feature = "arbitrary"))]
fn check_arbitrary_feature(_data: &[u8]) {}

fn check_unit_f32_optional_features(_value: UnitInterval<f32>, _raw: f32) {
    #[cfg(feature = "bytemuck")]
    {
        use bytemuck::CheckedBitPattern as _;

        assert_eq!(
            UnitInterval::<f32>::is_valid_bit_pattern(&_raw),
            UnitInterval::<f32>::contains(_raw)
        );
        assert_eq!(
            bytemuck::checked::try_from_bytes::<UnitInterval<f32>>(bytemuck::bytes_of(&_raw))
                .is_ok(),
            UnitInterval::<f32>::contains(_raw)
        );
    }

    #[cfg(feature = "num-traits")]
    {
        use num_traits::{Bounded as _, FromPrimitive as _, One as _, ToPrimitive as _};

        assert_eq!(
            UnitInterval::<f32>::from_f32(_raw),
            UnitInterval::<f32>::new(_raw)
        );
        assert_eq!(UnitInterval::<f32>::min_value(), UnitInterval::<f32>::ZERO);
        assert_eq!(UnitInterval::<f32>::max_value(), UnitInterval::<f32>::ONE);
        assert!(UnitInterval::<f32>::one().is_one());
        assert_eq!(_value.to_f32(), Some(_value.get()));
    }

    #[cfg(feature = "serde")]
    {
        let json = serde_json::to_string(&_value).unwrap();

        assert_eq!(
            serde_json::from_str::<UnitInterval<f32>>(&json).unwrap(),
            _value
        );

        if _raw.is_finite() {
            let json = _raw.to_string();

            assert_eq!(
                serde_json::from_str::<UnitInterval<f32>>(&json).is_ok(),
                UnitInterval::<f32>::contains(_raw)
            );
        }
    }

    #[cfg(feature = "rkyv")]
    {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&_value).unwrap();
        let archived =
            rkyv::access::<rkyv::Archived<UnitInterval<f32>>, rkyv::rancor::Error>(&bytes).unwrap();
        let round_tripped =
            rkyv::deserialize::<UnitInterval<f32>, rkyv::rancor::Error>(archived).unwrap();

        assert_eq!(round_tripped, _value);
    }
}

fn check_signed_f32_optional_features(_value: SignedUnitInterval<f32>, _raw: f32) {
    #[cfg(feature = "bytemuck")]
    {
        use bytemuck::CheckedBitPattern as _;

        assert_eq!(
            SignedUnitInterval::<f32>::is_valid_bit_pattern(&_raw),
            SignedUnitInterval::<f32>::contains(_raw)
        );
        assert_eq!(
            bytemuck::checked::try_from_bytes::<SignedUnitInterval<f32>>(bytemuck::bytes_of(&_raw))
                .is_ok(),
            SignedUnitInterval::<f32>::contains(_raw)
        );
    }

    #[cfg(feature = "num-traits")]
    {
        use num_traits::{Bounded as _, FromPrimitive as _, One as _, ToPrimitive as _};

        assert_eq!(
            SignedUnitInterval::<f32>::from_f32(_raw),
            SignedUnitInterval::<f32>::new(_raw)
        );
        assert_eq!(
            SignedUnitInterval::<f32>::min_value(),
            SignedUnitInterval::<f32>::NEG_ONE
        );
        assert_eq!(
            SignedUnitInterval::<f32>::max_value(),
            SignedUnitInterval::<f32>::ONE
        );
        assert!(SignedUnitInterval::<f32>::one().is_one());
        assert_eq!(_value.to_f32(), Some(_value.get()));
    }

    #[cfg(feature = "serde")]
    {
        let json = serde_json::to_string(&_value).unwrap();

        assert_eq!(
            serde_json::from_str::<SignedUnitInterval<f32>>(&json).unwrap(),
            _value
        );

        if _raw.is_finite() {
            let json = _raw.to_string();

            assert_eq!(
                serde_json::from_str::<SignedUnitInterval<f32>>(&json).is_ok(),
                SignedUnitInterval::<f32>::contains(_raw)
            );
        }
    }

    #[cfg(feature = "rkyv")]
    {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&_value).unwrap();
        let archived =
            rkyv::access::<rkyv::Archived<SignedUnitInterval<f32>>, rkyv::rancor::Error>(&bytes)
                .unwrap();
        let round_tripped =
            rkyv::deserialize::<SignedUnitInterval<f32>, rkyv::rancor::Error>(archived).unwrap();

        assert_eq!(round_tripped, _value);
    }
}
