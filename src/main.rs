use std::path::Path;
use neko_ibkr_api::client::IbkrClient;
use neko_marketdata::{MarketData, Step, TimeSeries, to_log_return_series};
use neko_math::{compute_stats, first_passage_probability, generate_paths, MonteCarloParameters};
use fetch_market_data_history::fetch_and_save_marketdata_history;
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;

#[allow(dead_code)] mod frequency_distribution_analysis;
#[allow(dead_code)] mod plot_distribution;
mod fetch_market_data_history;

fn main() {
    let seed = 42;
    let n_paths = 1000;
    // Paths are generated in batches to keep memory bounded: one path is n_steps * 8 bytes
    let batch_size = 10;
    let n_steps = 10000;
    let time_slots:f64 = 100.0;
    let std = 0.09 / (time_slots.sqrt());
    let price:f64 = 100.0;
    let strike:f64 = 102.0;
    let threshold = strike.ln() - price.ln();

    let mut count = 0;
    for batch in 0..n_paths / batch_size {
        // Each batch needs its own seed, otherwise all batches would draw the same paths
        let params = MonteCarloParameters {
            paths_number: batch_size,
            steps_number: n_steps,
            mean: 0.0,
            std_dev: std,
            seed: seed + batch as u64,
        };
        for path in generate_paths(&params) {
            for step in 0..n_steps {
                if path.returns[step] > threshold {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("{} out of {} paths have one value above {}", count, n_paths, threshold);
    let proba = first_passage_probability(price , strike, n_steps as f64, 0.0, std);
    println!("Computed proba {} ", proba);
}
