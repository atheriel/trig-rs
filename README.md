# trig-rs

This is small project to fool around with Rust's super-powered enums. My aim is
to allow working with different measures of angles in the Euclidean plane with
full type safety and automatic conversion. Of course, it would be no fun to do
this for degrees and radians alone --- and so I'm also working on incorporating
the more estoric [gradians](https://en.wikipedia.org/wiki/Gradian),
[turns](https://en.wikipedia.org/wiki/Turn_(geometry)), and clock face systems
as well.

## Examples

```rust
use trig::{Angle, Trigonometry, sin, cos, tan};

// Angle can be constructed in both common formats:
let angle1: Angle<f64> = Angle::degrees(180.0);
let angle2: Angle<f64> = Angle::radians(Float::pi());

// As well as some more estoric ones:
let angle3: Angle<f64> = Angle::gradians(200.0);
let angle4: Angle<f64> = Angle::turns(0.5);

// And convert between them seemlessly:
match angle4.to_radians() {
    Rad(val) => println!("0.5 turns is {}!", Rad(val)),
    _ => fail!("But I wanted radians!")
}

// We can use the top-level trigonometric functions on any of them:
assert_eq!(sin(angle1), sin(angle2));
assert_eq!(cos(angle3), cos(angle4));

// We can also concatenate angles using Rust's + and - syntax, which will
// automatically handle conversion between different angle formats:
let angle5 = angle1 + angle2 - angle3;
assert_eq!(angle1, angle5);

// Note that angles are guaranteed to fall in the domains you'd expect
// them to:
assert_eq!(angle1, angle1 + angle1 + angle1)
```