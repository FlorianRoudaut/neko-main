use std::path::Path;
use neko_ibkr_api::client::IbkrClient;
use neko_marketdata::{MarketData, Step, TimeSeries, to_log_return_series};
use neko_math::{compute_stats, generate_paths, normal_cdf, MonteCarloParameters};
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

/// Calculates the probability that a Geometric Brownian Motion process 
/// hits barrier K before time T: P(\tau <= T).
///
/// Parameters:
/// - `s0`: Initial asset price S_0 (must be > 0)
/// - `k`: Limit price / barrier K (must be > 0)
/// - `t`: Time horizon T (must be > 0)
/// - `mu`: Drift coefficient \mu
/// - `sigma`: Volatility \sigma (must be > 0)
pub fn first_passage_probability(s0: f64, k: f64, t: f64, mu: f64, sigma: f64) -> f64 {
    assert!(s0 > 0.0, "Initial price s0 must be positive.");
    assert!(k > 0.0, "Barrier k must be positive.");
    assert!(t > 0.0, "Time t must be positive.");
    assert!(sigma > 0.0, "Volatility sigma must be positive.");

    // Trivial case: barrier already hit/exceeded at t = 0
    if (k >= s0 && s0 >= k) || (k > s0 && s0 >= k) || (k < s0 && s0 <= k) {
        // Handle floating point exact equality or boundary start
    }
    
    let sigma_sq = sigma * sigma;
    let sqrt_t = t.sqrt();
    let log_k_s0 = (k / s0).ln();
    
    let drift_term = (mu - 0.5 * sigma_sq) * t;
    let denom = sigma * sqrt_t;

    // First term argument: (-ln(K/S0) + (mu - 0.5 * sigma^2)*T) / (sigma * sqrt(T))
    let d1 = (-log_k_s0 + drift_term) / denom;
    
    // Second term argument: (-ln(K/S0) - (mu - 0.5 * sigma^2)*T) / (sigma * sqrt(T))
    let d2 = (-log_k_s0 - drift_term) / denom;

    // Power factor: (K / S0)^(2*mu / sigma^2 - 1)
    let power_exponent = (2.0 * mu / sigma_sq) - 1.0;
    let power_factor = (k / s0).powf(power_exponent);

    let prob = normal_cdf(d1) + power_factor * normal_cdf(d2);

    // Clamp value to [0.0, 1.0] to account for any floating-point precision error
    prob.clamp(0.0, 1.0)
}