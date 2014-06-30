use std::fmt;

/// Base floating point types
pub trait BaseFloat: Primitive + fmt::Show + fmt::Float + Float + FloatMath {}

impl BaseFloat for f32 {}
impl BaseFloat for f64 {}

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
    Clock {hour: S, minute: S, second: S},
}
 
impl<S: BaseFloat> Angle<S> {
    /// Returns an angle in radians.
    pub fn radians(s: S) -> Angle<S> { Rad(s) }
    
    /// Returns an angle in degrees.
    pub fn degrees(s: S) -> Angle<S> { Deg(s) }

    /// Converts an angle to radians.
    pub fn to_radians(&self) -> Angle<S> {
        match self {
            &Rad(val) => Angle::radians(val),
            &Deg(val) => Angle::radians(val.to_radians()),
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
