use std::error::Error;
use nwp_regime_analyzer::{
    types::RegimeType,
    weathers::WeatherAnalyzer,
    plotter::WeatherPlotter,
};

fn main() -> Result<(), Box<dyn Error>> {
    let grid_size = 50;
    let mut analyzer = WeatherAnalyzer::new(grid_size);

    let regimes = [
        (RegimeType::Zonal, "regime_zonal.png"),
        (RegimeType::Blocage, "regime_blocage.png"),
        (RegimeType::AnticycloneGroenlandais, "regime_anticyclone.png"),
        (RegimeType::ZonalMou, "regime_zonal_mou.png"),
    ];

    for (regime, filename) in regimes.iter() {
        analyzer.generate_regime(*regime);
        let plotter = WeatherPlotter::new(&analyzer);
        plotter.plot_regime(filename, *regime)?;
    }

    println!("Images générées avec succès !");
    Ok(())
}