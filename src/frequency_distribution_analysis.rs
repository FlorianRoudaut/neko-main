use std::path::Path;
use neko_marketdata::{MarketData, Step, to_log_return_series, compute_stats, normal_cdf_general};
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;

pub fn frequency_distribution_analysis() {
    let data_dir = Path::new("../data");
    let marketdata_repo: LocalFileRepository<MarketData> = LocalFileRepository::<MarketData>::load(data_dir)
        .expect("Failed to load MarketData repository");
    let loaded_md = marketdata_repo.read_all();
    let log_returns = to_log_return_series("AAPL log returns", Step::Day1, &loaded_md);

    let stats = compute_stats(&log_returns).expect("Not enough data to compute stats");
    println!("Returns mean {} and std {}", stats.mean, stats.std);

    let returns = &log_returns.values;
    let std_val = stats.std;
    let thresholds = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0];
    let labels = ["< 0.5 std", "< 1.0 std", "< 1.5 std", "< 2.0 std", "< 2.5 std", "< 3.0 std", "< 4.0 std", "< 5.0 std", "total"];

    let mut counts = [0usize; 9];
    for &r in returns {
        let abs_r = r.abs();
        for (i, &t) in thresholds.iter().enumerate() {
            if abs_r < t * std_val { counts[i] += 1; }
        }
        counts[8] += 1;
    }

    let total = returns.len() as f64;
    println!("\nReturn distribution:");
    println!("  {:<12} {:>8} {:>10} {:>12} {:>10}", "label", "count", "empirical%", "normal_cdf%", "diff");
    for i in 0..8 {
        let threshold = thresholds[i] * std_val;
        let cdf = normal_cdf_general(threshold, stats.mean, stats.std)
                - normal_cdf_general(-threshold, stats.mean, stats.std);
        let frequency = counts[i] as f64 / total;
        println!("  {:<12} {:>8} {:>9.2}% {:>11.2}% {:>9.2}%",
            labels[i], counts[i], frequency * 100.0, cdf * 100.0, (frequency - cdf) * 100.0);
    }
    println!("  {:<12} {:>8} {:>9.2}% 0", labels[8], counts[8], counts[8] as f64 / total * 100.0);
}
