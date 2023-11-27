use anyhow::{anyhow,Result};

const EARTH_RAD: f64 = 6371.0;

struct CParser {
    latitude: f64,
    longitude: f64,
}

impl CParser {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
    fn dist(&self, coord2: &CParser) -> f64 {
        let delta_lat = (coord2.latitude - self.latitude).to_radians();
        let delta_lon = (coord2.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
            self.latitude.to_radians().cos() * coord2.latitude.to_radians().cos() *
                (delta_lon / 2.0).sin() * (delta_lon / 2.0).sin();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RAD * c
    }

}
#[cfg(test)]
mod coordi_tests{
    use crate::coordinate_parser::cparser::{CParser};
    #[test]
    fn test_all() {
        let coord1 = CParser::new(23.8271222, 72.7810079);
        let coord2 = CParser::new(23.4775879, 72.3350619);
        let distance = coord1.dist(&coord2);
        assert_eq!(distance,59.78040903151486);
    }
}
