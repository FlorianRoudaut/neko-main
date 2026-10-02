use textplots::{Chart, LabelBuilder, LabelFormat, Plot, Shape};
use neko_marketdata::TimeSeries;

pub fn plot_distribution(ts: &TimeSeries) {
    let min = ts.values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = ts.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    const N_BINS: usize = 60;
    let bin_width = (max - min) / N_BINS as f64;
    let mut counts = vec![0u32; N_BINS];
    for &v in &ts.values {
        let i = ((v - min) / bin_width) as usize;
        counts[i.min(N_BINS - 1)] += 1;
    }

    let bars: Vec<(f32, f32)> = counts.iter().enumerate()
        .map(|(i, &c)| ((min + (i as f64 + 0.5) * bin_width) as f32, c as f32))
        .collect();

    println!("\nDistribution of '{}':", ts.name);
    Chart::new(180, 60, min as f32, max as f32)
        .x_label_format(LabelFormat::Custom(Box::new(|v| format!("{:.4}", v))))
        .lineplot(&Shape::Bars(&bars))
        .nice();
}
