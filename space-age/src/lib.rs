// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

macro_rules! planet {
    ($name: ident, $period:expr) => {
        pub struct $name;

        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                Self::convert_years(d, $period)
            }
        }
    };
}

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;
const MERCURY_PERIOD: f64 = 0.2408467;
const VENUS_PERIOD: f64   = 0.61519726;
const EARTH_PERIOD: f64   = 1.0;
const MARS_PERIOD: f64    = 1.8808158;
const JUPITER_PERIOD: f64 = 11.862615;
const SATURN_PERIOD: f64  = 29.447498;
const URANUS_PERIOD: f64  = 84.016846;
const NEPTUNE_PERIOD: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    years_on_earth: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let s = s as f64;

        Self { years_on_earth: s / EARTH_YEAR_SECONDS }
    }
}

pub trait Planet {
    fn convert_years(d: &Duration, planet_revolution_duration: f64) -> f64 {
        d.years_on_earth / planet_revolution_duration
    }

    fn years_during(d: &Duration) -> f64;
}

planet!(Mercury, MERCURY_PERIOD);
planet!(Venus, VENUS_PERIOD);
planet!(Earth, EARTH_PERIOD);
planet!(Mars, MARS_PERIOD);
planet!(Jupiter, JUPITER_PERIOD);
planet!(Saturn, SATURN_PERIOD);
planet!(Uranus, URANUS_PERIOD);
planet!(Neptune, NEPTUNE_PERIOD);
