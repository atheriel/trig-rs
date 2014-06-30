/*
    This file is part of rs-noise, a procedural noise generation library.
*/

#![crate_id   = "trig#0.0.1-pre"]
#![comment = "Provides trigonometric primitives."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

// #![warn(missing_doc)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]

use angle::{BaseFloat, Angle, Trigonometry};

pub mod angle;

/// Calculate the sine.
#[inline] pub fn sin<S: BaseFloat>(a: Angle<S>) -> S { a.sin() }

/// Calculate the cosine.
#[inline] pub fn cos<S: BaseFloat>(a: Angle<S>) -> S { a.cos() }

/// Calculate the tangent.
#[inline] pub fn tan<S: BaseFloat>(a: Angle<S>) -> S { a.tan() }

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
