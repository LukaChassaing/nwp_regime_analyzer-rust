use std::f64::consts::PI;
use crate::types::{RegimeType, WeatherFields};

pub struct WeatherAnalyzer {
    pub fields: WeatherFields,
    grid_size: usize,
}

impl WeatherAnalyzer {
    pub fn new(grid_size: usize) -> Self {
        Self {
            fields: WeatherFields::new(grid_size),
            grid_size,
        }
    }

    pub fn lat_from_y(&self, y: usize) -> f64 {
        35.0 + (y as f64 / self.grid_size as f64) * 40.0
    }

    pub fn lon_from_x(&self, x: usize) -> f64 {
        -24.0 + (x as f64 / self.grid_size as f64) * 54.0
    }

    pub fn generate_regime(&mut self, regime: RegimeType) {
        match regime {
            RegimeType::Zonal => self.generate_zonal_regime(),
            RegimeType::Blocage => self.generate_blocking_regime(),
            RegimeType::AnticycloneGroenlandais => self.generate_greenland_anticyclone(),
            RegimeType::ZonalMou => self.generate_weak_zonal_regime(),
        }
    }

    fn generate_zonal_regime(&mut self) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                let lat = self.lat_from_y(i);
                let wind_intensity = match lat {
                    lat if (47.0..53.0).contains(&lat) => 50.0,
                    lat if (43.0..57.0).contains(&lat) => 30.0,
                    _ => 15.0,
                };

                self.fields.wind_u[i][j] = wind_intensity;
                self.fields.wind_v[i][j] = 0.0;
            }
        }
    }

    fn generate_blocking_regime(&mut self) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                let lat = self.lat_from_y(i);
                let lon = self.lon_from_x(j);
                let (dist, angle) = self.calculate_distance_angle(lat, lon, 50.0, 0.0);

                let intensity = 25.0 * f64::exp(-dist.powi(2) / 500.0);
                self.fields.wind_u[i][j] = -intensity * f64::sin(angle);
                self.fields.wind_v[i][j] = intensity * f64::cos(angle);
                self.fields.geopotential[i][j] = 5600.0 + 300.0 * f64::exp(-dist.powi(2) / 800.0);
            }
        }
    }

    fn generate_greenland_anticyclone(&mut self) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                let lat = self.lat_from_y(i);
                let lon = self.lon_from_x(j);
                let (dist, angle) = self.calculate_distance_angle(lat, lon, 65.0, -20.0);

                let intensity = 20.0 * f64::exp(-dist.powi(2) / 600.0);
                self.fields.wind_u[i][j] = -intensity * f64::sin(angle);
                self.fields.wind_v[i][j] = intensity * f64::cos(angle);
                self.fields.geopotential[i][j] = 5600.0 + 400.0 * f64::exp(-dist.powi(2) / 600.0);
            }
        }
    }

    fn generate_weak_zonal_regime(&mut self) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                let lat = self.lat_from_y(i);
                let lon = self.lon_from_x(j);
                let wave = 8.0 * f64::sin(2.0 * PI * (lon + 24.0) / 54.0);

                let wind_intensity = 10.0 * f64::exp(-(lat - 50.0 + wave).powi(2) / 200.0);
                self.fields.wind_u[i][j] = wind_intensity;
                self.fields.wind_v[i][j] = wave * 0.5;
                self.fields.geopotential[i][j] = 5600.0 +
                    200.0 * f64::sin(lat * PI / 180.0) +
                    50.0 * wave;
            }
        }
    }

    fn calculate_distance_angle(&self, lat: f64, lon: f64, center_lat: f64, center_lon: f64) -> (f64, f64) {
        let dist = f64::sqrt((lat - center_lat).powi(2) + (lon - center_lon).powi(2));
        let angle = f64::atan2(lat - center_lat, lon - center_lon);
        (dist, angle)
    }
}