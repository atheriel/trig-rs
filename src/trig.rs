/*
    This file is part of trig-rs, a library for doing typesafe trigonometry
    with a variety of angle formats (radians, degrees, grad, turns, and so on).
*/

//! # `trig-rs`: Typesafe Trigonometric Primitives
//!
//! Leverage Rust's super-powered enums to create a typesafe system for
//! trigonometry in degrees, radians, and more.
//!
//! ## Examples
//!
//! ```rust
//! use trig::{Angle, sin, cos};
//!
//! // Angle can be constructed in both common formats:
//! let angle1: Angle<f64> = Angle::degrees(180.0);
//! let angle2: Angle<f64> = Angle::radians(Float::pi());
//!
//! // As well as some more estoric ones:
//! let angle3: Angle<f64> = Angle::gradians(200.0);
//! let angle4: Angle<f64> = Angle::turns(0.5);
//!
//! // And convert between them seemlessly:
//! match angle4.to_radians() {
//!     Rad(val) => println!("0.5 turns is {}!", Rad(val)),
//!     _ => fail!("But I wanted radians!")
//! }
//!
//! // We can use the top-level trigonometric functions on any of them:
//! assert_eq!(sin(angle1), sin(angle2));
//! assert_eq!(cos(angle3), cos(angle4));
//!
//! // We can also concatenate angles using Rust's + and - syntax, which will
//! // automatically handle conversion between different angle formats:
//! let angle5 = angle1 + angle2 - angle3;
//! assert_eq!(angle1, angle5);
//!
//! // Note that angles are guaranteed to fall in the domains you'd expect
//! // them to:
//! assert_eq!(angle1, angle1 + angle1 + angle1)
//! ```

#![crate_name = "trig"]
#![comment = "Provides trigonometric primitives."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![unstable]
#![feature(macro_rules)]
#![feature(struct_variant)]

use std::fmt;

/*
    Top-level functions.
*/

/// Calculate the sine.
#[stable] #[inline] pub fn sin<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.sin() }

/// Calculate the cosine.
#[stable] #[inline] pub fn cos<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.cos() }

/// Calculate the tangent.
#[stable] #[inline] pub fn tan<S: BaseFloat, T: Trigonometry<S>>(t: T) -> S { t.tan() }

/// Calculate the arcsine (in radians).
#[inline] pub fn asin<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.asin()) }

/// Calculate the arccosine (in radians).
#[inline] pub fn acos<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.acos()) }

/// Calculate the arctangent (in radians).
#[inline] pub fn atan<S: BaseFloat>(s: S) -> Angle<S> { Angle::radians(s.atan()) }

/*
    The Trigonometry trait.
*/

/// Represents an object for which trigonometric methods are sensible and return
/// values of type `S`.
#[stable]
pub trait Trigonometry<S> {
    /// Compute the sine of the object.
    fn sin(&self) -> S;
    /// Compute the cosine of the object.
    fn cos(&self) -> S;
    /// Compute the tangent of the object.
    fn tan(&self) -> S;
    // /// Compute the cosecant of the object.
    // fn csc(&self) -> S;
    // /// Compute the secant of the object.
    // fn sec(&self) -> S;
    // /// Compute the cotangent of the object.
    // fn cot(&self) -> S;
}

/*
    The Angle enum and its implementations.
*/

/// Base floating point types
pub trait BaseFloat: Primitive + FromPrimitive + fmt::Show + fmt::Float + Float + FloatMath {}

impl BaseFloat for f32 {}
impl BaseFloat for f64 {}

/// Encompasses representations of angles in the Euclidean plane.
#[deriving(Clone, PartialEq, PartialOrd, Hash)]
pub enum Angle<S> {
    /// An angle in radians.
    #[stable] Rad(S),
    /// An angle in degrees.
    #[stable] Deg(S),
    /// An angle in [gradians](http://en.wikipedia.org/wiki/Grad_(angle)).
    #[stable] Grad(S),
    /// An angle in [turns](http://en.wikipedia.org/wiki/Turn_(geometry)).
    #[stable] Turn(S),
    /// An angle as it would appear on the face of a clock.
    #[experimental] Clock {
        /// The hours portion.
        pub hour: S,
        /// The minutes portion.
        pub minute: S,
        /// The seconds portion.
        pub second: S
    },
}
 
impl<S: BaseFloat + Mul<S, S> + Div<S, S> + Rem<S, S>> Angle<S> {
    /// Returns an angle in radians.
    pub fn radians(s: S) -> Angle<S> { Rad(s % Float::two_pi()) }
    
    /// Returns an angle in degrees.
    pub fn degrees(s: S) -> Angle<S> { Deg(s % FromPrimitive::from_f64(360.0).unwrap()) }

    /// Returns an angle in gradians.
    pub fn gradians(s: S) -> Angle<S> { Grad(s % FromPrimitive::from_f64(400.0).unwrap()) }

    /// Returns an angle in turns.
    pub fn turns(s: S) -> Angle<S> { Turn(s.fract()) }

    /// Returns an angle as it would appear on a clock.
    pub fn clock_face(hour: S, minute: S, second: S) -> Angle<S> {
        Clock { hour: hour, minute: minute, second: second }
    }

    /// Converts an angle to radians.
    pub fn to_radians(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::radians(val),
            &Deg(val) => Angle::radians(val.to_radians()),
            &Grad(val) => Angle::radians(val * Float::pi() / FromPrimitive::from_f64(200.0).unwrap()),
            &Turn(val) => Angle::radians(val * Float::two_pi()),
            _ => unimplemented!()
        }
    }

    /// Converts an angle to degrees.
    pub fn to_degrees(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::degrees(val.to_degrees()),
            &Deg(val) => Angle::degrees(val),
            &Grad(val) => Angle::degrees(val * FromPrimitive::from_f64(360.0 / 400.0).unwrap()),
            &Turn(val) => Angle::degrees(val * FromPrimitive::from_f64(360.0).unwrap()),
            _ => unimplemented!()
        }
    }

    /// Converts an angle to gradians.
    pub fn to_gradians(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::gradians(val / Float::pi() * FromPrimitive::from_f64(200.0).unwrap()),
            &Deg(val) => Angle::gradians(val * FromPrimitive::from_f64(400.0 / 360.0).unwrap()),
            &Grad(val) => Angle::gradians(val),
            &Turn(val) => Angle::gradians(val * FromPrimitive::from_f64(400.0).unwrap()),
            _ => unimplemented!()
        }
    }

    /// Converts an angle to turns.
    pub fn to_turns(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::turns(val / Float::two_pi()),
            &Deg(val) => Angle::turns(val / FromPrimitive::from_f64(360.0).unwrap()),
            &Grad(val) => Angle::turns(val / FromPrimitive::from_f64(400.0).unwrap()),
            &Turn(val) => Angle::turns(val),
            _ => unimplemented!()
        }
    }

    /// One half of the domain. In radians, this is `π`.
    pub fn half() -> Angle<S> { Rad(Float::pi()) }

    /// One quarter of the domain. In radians, this is `π/2`.
    pub fn quarter() -> Angle<S> { Rad(Float::frac_pi_2()) }

    /// One sixth of the domain. In radians, this is `π/3`.
    pub fn sixth() -> Angle<S> { Rad(Float::frac_pi_3()) }

    /// One eighth of the domain. In radians, this is `π/4`.
    pub fn eighth() -> Angle<S> { Rad(Float::frac_pi_4()) }

    /// Gets the raw value that is stored in the angle.
    ///
    /// ## Failure
    ///
    /// Clock-valued angles are not encoded as a single value, and so this
    /// method will always fail for them.
    pub fn unwrap(&self) -> S {
        match self {
            &Rad(s)|&Deg(s)|&Grad(s)|&Turn(s) => s,
            _ => fail!("Clock values cannot be unwrapped.")
        }
    }
}

impl<S: BaseFloat> Add<Angle<S>, Angle<S>> for Angle<S> {
    #[inline]
    fn add(&self, other: &Angle<S>) -> Angle<S> {
        match (self, other) {
            (&Rad(val), othr) => Angle::radians(val + othr.to_radians().unwrap()),
            (&Deg(val), othr) => Angle::degrees(val + othr.to_degrees().unwrap()),
            (&Grad(val), othr) => Angle::gradians(val + othr.to_gradians().unwrap()),
            (&Turn(val), othr) => Angle::turns(val + othr.to_turns().unwrap()),
            _ => unimplemented!()
        }
    }
}

impl<S: BaseFloat> Sub<Angle<S>, Angle<S>> for Angle<S> {
    #[inline]
    fn sub(&self, other: &Angle<S>) -> Angle<S> {
        match (self, other) {
            (&Rad(val), othr) => Angle::radians(val - othr.to_radians().unwrap()),
            (&Deg(val), othr) => Angle::degrees(val - othr.to_degrees().unwrap()),
            (&Grad(val), othr) => Angle::gradians(val - othr.to_gradians().unwrap()),
            (&Turn(val), othr) => Angle::turns(val - othr.to_turns().unwrap()),
            _ => unimplemented!()
        }
    }
}

impl<S: BaseFloat + fmt::Show> fmt::Show for Angle<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Deg(val) => write!(f, "{}°", val),
            &Rad(val) => write!(f, "{} rad", val),
            &Grad(val) => write!(f, "{} gon", val),
            &Turn(val) => write!(f, "{} turns", val),
            _ => fail!("Not yet implemented.")
        }
    }
}

macro_rules! angle_trigonometry (
    ($($method:ident),+ ) => (
        impl<S: BaseFloat> Trigonometry<S> for Angle<S> {
            $(fn $method(&self) -> S {
                match self {
                    &Rad(val) => val.$method(),
                    &other => other.to_radians().$method()
                }
            }
            )+
        }
    )
)

angle_trigonometry!(sin, cos, tan)

/*
    Test suite.
*/

#[cfg(test)]
mod test {
    use super::Angle;

    #[test]
    fn test_conversion() {
        let half: Angle<f64> = Angle::half();
        assert_eq!(half.to_degrees().to_gradians().to_turns().to_radians(), half);
        assert_eq!(half.to_turns().to_gradians().to_degrees().to_radians(), half);
        assert_eq!(half.to_degrees().to_turns().to_gradians().to_radians(), half);
        assert_eq!(half.to_gradians().to_radians(), half);
    }

    #[test]
    fn test_operators() {
        assert_eq!(Angle::degrees(100.0f64) + Angle::degrees(100.0f64), Angle::degrees(200.0f64));
        assert_eq!(Angle::degrees(100.0f64) - Angle::degrees(100.0f64), Angle::degrees(0.0f64));
        assert_eq!(Angle::degrees(100.0f64) + Angle::radians(0.0f64), Angle::degrees(100.0f64));
        assert_eq!(Angle::radians(1.0f64) - Angle::degrees(0.0f64), Angle::radians(1.0f64));
    }
}
