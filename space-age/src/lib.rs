#[macro_use]
macro_rules! planet {
    ($name:ident, $seconds:expr) => {
        pub struct $name;
        impl $name {
            pub fn years_during(d: &Duration) -> f64 {
                d / $seconds
            }
        }
    };
}

const EARTH     : f64 = 31557600.0;
const MERCURY   : f64 = 0.2408467  * EARTH;
const VENUS     : f64 = 0.61519726 * EARTH;
const MARS      : f64 = 1.8808158  * EARTH;
const JUPITER   : f64 = 11.862615  * EARTH;
const SATURN    : f64 = 29.447498  * EARTH;
const URANUS    : f64 = 84.016846  * EARTH;
const NEPTUNE   : f64 = 164.79132  * EARTH;

pub type Duration = f64;

planet!(Venus, VENUS);
planet!(Mercury, MERCURY);
planet!(Earth, EARTH);
planet!(Mars, MARS);
planet!(Jupiter, JUPITER);
planet!(Saturn, SATURN);
planet!(Uranus, URANUS);
planet!(Neptune, NEPTUNE);
