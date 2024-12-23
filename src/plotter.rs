use plotters::prelude::*;
use std::error::Error;
use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use crate::types::RegimeType;
use crate::weathers::WeatherAnalyzer;

pub struct WeatherPlotter<'a> {
    analyzer: &'a WeatherAnalyzer,
}

impl<'a> WeatherPlotter<'a> {
    pub fn new(analyzer: &'a WeatherAnalyzer) -> Self {
        Self { analyzer }
    }

    pub fn plot_regime(&self, filename: &str, regime: RegimeType) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = self.create_chart(&root, &regime)?;
        self.draw_map_background(&mut chart)?;
        self.draw_wind_vectors(&mut chart, regime)?;

        root.present()?;
        Ok(())
    }

    fn create_chart<'b, DB: DrawingBackend>(
        &'b self,
        root: &'b DrawingArea<DB, Shift>,
        regime: &RegimeType,
    ) -> Result<ChartContext<'b, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>, Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        ChartBuilder::on(root)
            .caption(regime.to_string(), ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                0f64..self.analyzer.fields.wind_u.len() as f64,
                0f64..self.analyzer.fields.wind_u.len() as f64,
            )
            .map(|mut chart| {
                chart
                    .configure_mesh()
                    .x_labels(12)
                    .y_labels(8)
                    .x_label_formatter(&|x| {
                        format!("{}°E", self.analyzer.lon_from_x(*x as usize) as i32)
                    })
                    .y_label_formatter(&|y| {
                        format!("{}°N", self.analyzer.lat_from_y(*y as usize) as i32)
                    })
                    .axis_style(RGBColor(50, 50, 50))
                    .light_line_style(RGBColor(200, 200, 200))
                    .draw()?;
                Ok(chart)
            })?
    }

    fn draw_map_background<DB: DrawingBackend>(
        &self,
        chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> Result<(), Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        let img = image::open("europe_map.png")?.to_rgba8();
        let (img_width, img_height) = img.dimensions();
        let plotting_area = chart.plotting_area();

        let (x_range, y_range) = (plotting_area.get_pixel_range().0, plotting_area.get_pixel_range().1);
        let (plot_width, plot_height) = ((x_range.end - x_range.start) as u32, (y_range.end - y_range.start) as u32);

        for y in 0..plot_height {
            for x in 0..plot_width {
                let img_x = (x as f64 * img_width as f64 / plot_width as f64) as u32;
                let img_y = (img_height - 1) - ((y as f64 * img_height as f64 / plot_height as f64) as u32);

                if img_x < img_width && img_y < img_height {
                    let pixel = img.get_pixel(img_x, img_y);
                    if pixel[3] > 0 {
                        let grid_x = x as f64 * self.analyzer.fields.wind_u.len() as f64 / plot_width as f64;
                        let grid_y = y as f64 * self.analyzer.fields.wind_u.len() as f64 / plot_height as f64;
                        plotting_area.draw_pixel((grid_x, grid_y), &RGBColor(pixel[0], pixel[1], pixel[2]))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_wind_vectors<DB: DrawingBackend>(
        &self,
        chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        regime: RegimeType,
    ) -> Result<(), Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        let blue_color = RGBColor(0, 0, 255);
        let step = regime.step_size();
        let base_scale = regime.scale_factor();

        for i in (0..self.analyzer.fields.wind_u.len()).step_by(step) {
            for j in (0..self.analyzer.fields.wind_u[0].len()).step_by(step) {
                let (x, y) = (j as f64, i as f64);
                let (u, v) = (
                    self.analyzer.fields.wind_u[i][j],
                    -self.analyzer.fields.wind_v[i][j]
                );

                let speed = f64::sqrt(u * u + v * v);
                if speed > 0.1 {
                    let scale = if matches!(regime, RegimeType::Zonal) && speed > 40.0 {
                        base_scale * 1.2
                    } else {
                        base_scale
                    };

                    let (dx, dy) = (u * scale / speed, v * scale / speed);
                    self.draw_vector(chart, x, y, dx, dy, &blue_color, regime, speed > 40.0)?;
                }
            }
        }
        Ok(())
    }

    fn draw_vector<DB: DrawingBackend>(
        &self,
        chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        x: f64,
        y: f64,
        dx: f64,
        dy: f64,
        color: &RGBColor,
        regime: RegimeType,
        draw_arrow: bool,
    ) -> Result<(), Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        chart.draw_series(LineSeries::new(
            vec![(x, y), (x + dx, y + dy)],
            color,
        ))?;

        if matches!(regime, RegimeType::Zonal) && draw_arrow {
            let arrow_head_size = regime.scale_factor() * 0.15;
            chart.draw_series(LineSeries::new(
                vec![
                    (x + dx - arrow_head_size, y + arrow_head_size),
                    (x + dx, y),
                    (x + dx - arrow_head_size, y - arrow_head_size)
                ],
                color,
            ))?;
        }
        Ok(())
    }
}