use std::io::Write;

use plotters::{backend::{BitMapBackend, RGBPixel}, chart::ChartBuilder, drawing::IntoDrawingArea, series::LineSeries, style::{RED, WHITE}};

use crate::softoned::SoftSimulator1D;


pub fn render_1d(sim: &SoftSimulator1D, writer: &mut std::process::ChildStdin, frame_buf: &mut [u8], width: u32, height: u32, length: f64) {
    {
        let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(frame_buf, (width, height)).unwrap().into_drawing_area();
        root.fill(&WHITE).unwrap();
        
        let dx = length / (sim.state.len() as f64);
        let max_prob = sim.state.iter().map(|c| c.norm_sqr()).fold(0.0f64, f64::max);
        let max_y = (max_prob * 1.5).max(0.1); // add some headroom
        
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-length/2.0..length/2.0, 0.0..max_y)
            .unwrap();
            
        chart.configure_mesh().draw().unwrap();
        
        chart.draw_series(LineSeries::new(
            sim.state.iter().enumerate().map(|(j, c)| {
                let x = (j as f64) * dx - (length / 2.0);
                (x, c.norm_sqr())
            }),
            &RED,
        )).unwrap();
        
        root.present().unwrap();
    }
    writer.write_all(frame_buf).unwrap();
}

