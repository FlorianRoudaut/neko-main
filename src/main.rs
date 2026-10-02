use std::path::Path;

use neko_ibkr_api::client::IbkrClient;
use neko_marketdata::{MarketData, Step, to_log_return_series};
use fetch_market_data_history::fetch_and_save_marketdata_history;
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;

#[allow(dead_code)] mod frequency_distribution_analysis;
#[allow(dead_code)] mod plot_distribution;
mod fetch_market_data_history;

fn main() {
    frequency_distribution_analysis::frequency_distribution_analysis();
}
