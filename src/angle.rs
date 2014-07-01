/*
    This file is part of trig-rs, a library for doing typesafe trigonometry
    with a variety of angle formats (radians, degrees, grad, turns, and so on).
*/

use std::fmt;

/// Base floating point types
pub trait BaseFloat: Primitive + fmt::Show + fmt::Float + Float + FloatMath {
    fn gradians_to_radians(self) -> Self;
    fn gradians_to_degrees(self) -> Self;
    fn gradians_to_turns(self) -> Self;
}

impl BaseFloat for f32 {
    fn gradians_to_radians(self) -> f32 {
        self * Float::pi() / 200.0f32
    }

    fn gradians_to_degrees(self) -> f32 {
        self * 360.0f32 / 400.0f32
    }

    fn gradians_to_turns(self) -> f32 {
        self / 400.0f32
    }
}

impl BaseFloat for f64 {
    fn gradians_to_radians(self) -> f64 {
        self * Float::pi() / 200.0f64
    }

    fn gradians_to_degrees(self) -> f64 {
        self * 360.0f64 / 400.0f64
    }

    fn gradians_to_turns(self) -> f64 {
        self / 400.0f64
    }
}

/// Encompasses representations of angles in the Euclidean plane.
#[deriving(Clone, PartialEq, PartialOrd, Hash)]
pub enum Angle<S> {
    /// An angle in radians.
    Rad(S),
    /// An angle in degrees.
    Deg(S),
    /// An angle in [gradians](http://en.wikipedia.org/wiki/Grad_(angle)).
    Grad(S),
    /// An angle in [turns](http://en.wikipedia.org/wiki/Turn_(geometry)).
    Turn(S),
    /// An angle as it would appear on the face of a clock.
    Clock {
        /// The hours portion.
        hour: S,
        /// The minutes portion.
        minute: S,
        /// The seconds portion.
        second: S
    },
}
 
impl<S: BaseFloat> Angle<S> {
    /// Returns an angle in radians.
    pub fn radians(s: S) -> Angle<S> { Rad(s) }
    
    /// Returns an angle in degrees.
    pub fn degrees(s: S) -> Angle<S> { Deg(s) }

    /// Returns an angle in gradians.
    pub fn gradians(s: S) -> Angle<S> { Grad(s) }

    /// Returns an angle in turns.
    pub fn turns(s: S) -> Angle<S> { Turn(s) }

    /// Returns an angle as it would appear on a clock.
    pub fn clock_face(hour: S, minute: S, second: S) -> Angle<S> {
        Clock { hour: hour, minute: minute, second: second }
    }

    /// Converts an angle to radians.
    pub fn to_radians(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::radians(val),
            &Deg(val) => Angle::radians(val.to_radians()),
            &Grad(val) => Angle::radians(val.gradians_to_radians()),
            _ => fail!("Not yet implemented.")
        }
    }

    /// Converts an angle to degrees.
    pub fn to_degrees(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::degrees(val.to_degrees()),
            &Deg(val) => Angle::degrees(val),
            _ => fail!("Not yet implemented.")
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
}

/// Represents an object for which trigonometric methods are sensible and return
/// values of type `S`.
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

macro_rules! angle_trigonometry (
    ($($method:ident),+ ) => (
        impl<S: BaseFloat> Trigonometry<S> for Angle<S> {
            $(fn $method(&self) -> S {
                match self {
                    &Rad(val) => val.$method(),
                    &Deg(val) => val.to_radians().$method(),
                    _ => fail!("Not yet implemented.")
                }
            }
            )+
        }
    )
)

angle_trigonometry!(sin, cos, tan)

macro_rules! angle_ops (
    ($Trait:ident, $method:ident) => (
        impl<S: BaseFloat> $Trait<Angle<S>, Angle<S>> for Angle<S> {
            #[inline]
            fn $method(&self, other: &Angle<S>) -> Angle<S> {
                match (self, other) {
                    (&Deg(s1), &Deg(s2)) => Angle::degrees(s1.$method(&s2)),
                    (&Deg(s1), &Rad(s2)) => Angle::degrees(s1.$method(&s2.to_degrees())),
                    (&Rad(s1), &Rad(s2)) => Angle::radians(s1.$method(&s2)),
                    (&Rad(s1), &Deg(s2)) => Angle::radians(s1.$method(&s2.to_radians())),
                    _ => fail!("Not yet implemented.")
                }
            }
        }
    )
)

angle_ops!(Add, add)
angle_ops!(Sub, sub)
angle_ops!(Mul, mul)
angle_ops!(Div, div)

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
