// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    second: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self{ second: s }
    }
}

pub trait Planet {
    const ORBITAL_SPEED: f64;

    fn years_during(d: &Duration) -> f64 {
        d.second as f64 / 31557600.0 / Self::ORBITAL_SPEED
    }
}

macro_rules! planet {
    ($planet:ident, $orbital_speed:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            const ORBITAL_SPEED: f64 = $orbital_speed;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);