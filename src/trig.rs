/*
    This file is part of trig-rs, a library for doing typesafe trigonometry
    with a variety of angle formats (radians, degrees, grad, turns, and so on).
*/

#![crate_id   = "trig#0.0.1-pre"]
#![comment = "Provides trigonometric primitives."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

// #![warn(missing_doc)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]

pub use self::angle::{BaseFloat, Angle, Trigonometry};

mod angle;

/// Calculate the sine.
#[inline] pub fn sin<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.sin() }

/// Calculate the cosine.
#[inline] pub fn cos<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.cos() }

/// Calculate the tangent.
#[inline] pub fn tan<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.tan() }

/// Calculate the arcsine (in radians).
#[inline] pub fn asin<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.asin()) }

/// Calculate the arccosine (in radians).
#[inline] pub fn acos<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.acos()) }

/// Calculate the arctangent (in radians).
#[inline] pub fn atan<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.atan()) }

#[cfg(test)]
mod test {
    use super::angle::*;

    #[test]
    fn test_conversion() {
        assert_eq!(Angle::degrees(-5.0f64).to_radians().to_degrees(), Angle::degrees(-5.0f64));
        assert_eq!(Angle::radians(-5.0f64).to_degrees().to_radians(), Angle::radians(-5.0f64));
        let half: Angle<f64> = Angle::half();
        assert_eq!(half.to_degrees().to_gradians().to_turns().to_radians(), half);
    }

    #[test]
    fn test_operators() {
        assert_eq!(Angle::degrees(100.0f64) + Angle::degrees(100.0f64), Angle::degrees(200.0f64));
        assert_eq!(Angle::degrees(100.0f64) - Angle::degrees(100.0f64), Angle::degrees(0.0f64));
        assert_eq!(Angle::degrees(100.0f64) + Angle::radians(0.0f64), Angle::degrees(100.0f64));
        assert_eq!(Angle::radians(1.0f64) - Angle::degrees(0.0f64), Angle::radians(1.0f64));
        assert_eq!(Angle::degrees(2.0f64) * Angle::degrees(100.0f64), Angle::degrees(200.0f64));
    }
}
