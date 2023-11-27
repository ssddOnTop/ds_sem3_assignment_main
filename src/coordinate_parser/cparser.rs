use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use anyhow::{anyhow, Result};
use serde_json::Value;

const EARTH_RAD: f64 = 6371.0;
#[derive(Debug,Clone,Copy,Default)]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Eq for Coordinates{}
impl PartialEq<Self> for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
    }
}

impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.longitude.to_bits() ^ self.latitude.to_bits()).hash(state)
    }
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
    pub fn dist(&self, coord2: &Coordinates) -> f64 {
        let delta_lat = (coord2.latitude - self.latitude).to_radians();
        let delta_lon = (coord2.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
            self.latitude.to_radians().cos() * coord2.latitude.to_radians().cos() *
                (delta_lon / 2.0).sin() * (delta_lon / 2.0).sin();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RAD * c
    }

    pub fn from(value: &Value) -> Option<(Self, String)> {
        let longitude:f64 = match value.get("longitude")?.as_str()?.parse() {
            Ok(v) => v,
            Err(_) => {
                value.get("longitude")?.as_f64()?
            }
        };
        let latitude:f64 = match value.get("latitude")?.as_str()?.parse() {
            Ok(v) => v,
            Err(_) => {
                value.get("latitude")?.as_f64()?
            }
        };
        // let latitude:f64 = value.get("latitude")?.as_str()?.parse().unwrap_or_default();
        let loc = value.get("location")?.as_str()?;
        Some((Self{ latitude, longitude }, loc.to_string()))
    }

}
#[cfg(test)]
mod coordi_tests{
    use crate::coordinate_parser::cparser::Coordinates;
    #[test]
    fn test_all() {
        let coord1 = Coordinates::new(23.8271222, 72.7810079);
        let coord2 = Coordinates::new(23.4775879, 72.3350619);
        let distance = coord1.dist(&coord2);
        assert_eq!(distance,59.78040903151486);
    }
}
