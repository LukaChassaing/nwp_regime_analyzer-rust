#[derive(Debug, Copy, Clone)]
pub enum RegimeType {
    Zonal,
    Blocage,
    AnticycloneGroenlandais,
    ZonalMou,
}

impl RegimeType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Zonal => "Zonal",
            Self::Blocage => "Blocage",
            Self::AnticycloneGroenlandais => "Anticyclone Groenlandais",
            Self::ZonalMou => "Zonal Mou",
        }
            .to_string()
    }

    pub fn step_size(&self) -> usize {
        match self {
            Self::Zonal => 4,
            Self::Blocage | Self::AnticycloneGroenlandais => 2,
            Self::ZonalMou => 3,
        }
    }

    pub fn scale_factor(&self) -> f64 {
        match self {
            Self::Zonal => 5.0,
            Self::Blocage | Self::AnticycloneGroenlandais => 3.0,
            Self::ZonalMou => 2.5,
        }
    }
}

#[derive(Debug)]
pub struct WeatherFields {
    pub geopotential: Vec<Vec<f64>>,
    pub wind_u: Vec<Vec<f64>>,
    pub wind_v: Vec<Vec<f64>>,
}

impl WeatherFields {
    pub fn new(grid_size: usize) -> Self {
        Self {
            geopotential: vec![vec![0.0; grid_size]; grid_size],
            wind_u: vec![vec![0.0; grid_size]; grid_size],
            wind_v: vec![vec![0.0; grid_size]; grid_size],
        }
    }
}