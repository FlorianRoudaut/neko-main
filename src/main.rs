use std::path::Path;

use neko_marketdata::{MarketData, Step, to_log_return_series};
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;

mod frequency_distribution_analysis;
mod plot_distribution;
use plot_distribution::plot_distribution;

fn main() {
    let data_dir = Path::new("../data");
    let marketdata_repo: LocalFileRepository<MarketData> = LocalFileRepository::<MarketData>::load(data_dir)
        .expect("Failed to load MarketData repository");
    let loaded_md = marketdata_repo.read_all();
    let log_returns = to_log_return_series("AAPL log returns", Step::Day1, &loaded_md);
    plot_distribution(&log_returns);
}
