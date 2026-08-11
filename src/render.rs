use std::io::Write;

use plotters::{backend::{BitMapBackend, RGBPixel}, chart::ChartBuilder, drawing::IntoDrawingArea, series::LineSeries, style::{RED, WHITE}};

use crate::{softoned::SoftSimulator1D, softtwod::SoftSimulator2D};


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

pub fn render_2d(sim: &SoftSimulator2D, writer: &mut std::process::ChildStdin, frame_buf: &mut [u8], width: usize, height: usize) {
    let mut max_prob = 1e-10;
    for c in sim.state.iter() {
        if c.norm_sqr() > max_prob { max_prob = c.norm_sqr(); }
    }
    
    for j in 0..height {
        for i in 0..width {
            let prob = sim.state[[i, j]].norm_sqr();
            let val = (prob / max_prob).clamp(0.0, 1.0);
            
            // Map to some color (heat map style)
            let r = (val * 255.0) as u8;
            let g = ((val.powf(2.0)) * 255.0) as u8;
            let b = ((val.powf(0.5)) * 255.0) as u8;
            
            let idx = (j * width + i) * 3;
            frame_buf[idx] = r;
            frame_buf[idx+1] = g;
            frame_buf[idx+2] = b;
        }
    }
    writer.write_all(frame_buf).unwrap();
}
