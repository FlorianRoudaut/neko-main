use std::path::Path;
use neko_ibkr_api::client::IbkrClient;
use neko_ibkr_api::marketdata_history::{HistoryRequest, get_marketdata_history_sync};
use neko_marketdata::{MarketData, Metric, Source};
use neko_securities::{Key, Security, Stock};
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;
use uuid::Uuid;

/*fn fetch_and_save_stock(client: &IbkrClient, ticker: &str) {
    let result = match get_contract_stock_sync(client, ticker) {
        Ok(r) => r,
        Err(e) => { eprintln!("Failed to fetch {} contract: {}", ticker, e); return; }
    };

    let stock_info = match result.get(ticker).and_then(|infos| infos.first()) {
        Some(info) => info,
        None => { eprintln!("No info returned for {}", ticker); return; }
    };

    let data_dir = Path::new("../data");
    let mut exchange_repo: LocalFileRepository<Exchange> = LocalFileRepository::<Exchange>::load(data_dir)
        .expect("Failed to load exchange repository");
    let mut stock_repo: LocalFileRepository<Stock> = LocalFileRepository::<Stock>::load(data_dir)
        .expect("Failed to load stock repository");

    for contract in &stock_info.contracts {

       let exchange =  Exchange {
                key: Key { id: Uuid::new_v4(), version: 0, unique_name: contract.exchange.clone() },
                name: contract.exchange.clone(),
        };

        let exchange_read_result = exchange_repo.read_one_by_name(&contract.exchange);
        if exchange_read_result.is_err() {
            let _res = exchange_repo.create(exchange);
        }

        let stock = Stock {
            key: Key {
                id: Uuid::new_v4(),
                version: 0,
                unique_name: format!("{}_{}", ticker, contract.exchange),
            },
            name: stock_info.name.clone(),
            bbg_id: String::new(),
            cusip: String::new(),
            isin: String::new(),
            ibkr_contract_id: contract.conid.to_string(),
            exchange_ids: vec![contract.exchange.clone()],
        };

        let stock_read_result = stock_repo.read_one_by_name(&stock.name);
        if stock_read_result.is_err() {
            let _res = stock_repo.create(stock);
        }
    }
}*/

fn timestamp_to_date(ts: u32) -> String {
    let days_per_month = [31u32, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days = ts / 86400;
    let mut year = 1970u32;

    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
        if days < days_in_year { break; }
        days -= days_in_year;
        year += 1;
    }

    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let mut month = 1u32;
    for &dim in &days_per_month {
        let dim = if month == 2 && leap { 29 } else { dim };
        if days < dim { break; }
        days -= dim;
        month += 1;
    }

    format!("{:04}-{:02}-{:02}", year, month, days + 1)
}

fn fetch_aapl_history(client: &IbkrClient) -> Vec<MarketData> {
    let data_dir = Path::new("../data");

    let mut source_repo: LocalFileRepository<Source> = LocalFileRepository::<Source>::load(data_dir)
        .expect("Failed to load source repository");
    let mut metric_repo: LocalFileRepository<Metric> = LocalFileRepository::<Metric>::load(data_dir)
        .expect("Failed to load metric repository");
    /*let mut stock_repo: LocalFileRepository<Stock> = LocalFileRepository::<Stock>::load(data_dir)
        .expect("Failed to load security repository");*/
    let apple_ibkr_contractid = 265598;
    let apple_id = uuid::Uuid::new_v4();

    let source = source_repo.read_all().into_iter().next();
    let close = metric_repo.read_one_by_name("Close");
    //let apple_stock = stock_repo.read_one_by_name("AAPL_NASDAQ").unwrap();

    let (source_rank, close_rank) = match (source, close) {
        (Some(s), Ok(m)) => (s.rank, m.rank),
        _ => {
            eprintln!("Source or Close metric not found in repository");
            return vec![];
        }
    };

    let req = HistoryRequest {
        conid: apple_ibkr_contractid,
        period: "7y",
        bar: "1d",
        outside_rth: false,
        start_time: Some("20200101-00:00:00"),
    };

    match get_marketdata_history_sync(client, req) {
        Ok(history) => {
            println!("AAPL history: {} bars", history.data.len());
            history.data.iter().map(|bar| MarketData {
                key: Key {
                    id: Uuid::new_v4(),
                    version: 0,
                    unique_name: String::new(),
                },
                security_id: apple_id,
                source: source_rank,
                metric_id: close_rank,
                value: bar.c,
                timestamp: (bar.t / 1000) as u32,
            }).collect()
        }
        Err(e) => {
            eprintln!("Failed to fetch AAPL history: {}", e);
            vec![]
        }
    }
}

fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sum: f64 = data.iter().sum();
    Some(sum / data.len() as f64)
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    let mean = mean(data)?;
    let count = data.len();

    // Population std dev uses `count as f64`, Sample std dev uses `(count - 1) as f64`
    if count < 2 {
        return None; // Standard deviation requires at least 2 data points for sample std dev
    }

    let variance = data
        .iter()
        .map(|value| {
            let diff = mean - value;
            diff * diff
        })
        .sum::<f64>()
        / (count - 1) as f64; // Use `count as f64` for population std dev

    Some(variance.sqrt())
}

fn main() {
    
    let data_dir = Path::new("../data");
    let marketdata_repo: LocalFileRepository<MarketData> = LocalFileRepository::<MarketData>::load(data_dir)
        .expect("Failed to load MarketData repository");
    let loaded_md = marketdata_repo.read_all();
    let mut returns: Vec<f64> = vec!();

    let mut previous = 0.0;
    for md in loaded_md{
        if(previous == 0.0) {
            previous = md.value;
        }

        let log_ret = md.value.ln() - previous.ln();
        println!("{};{};{}", timestamp_to_date(md.timestamp), md.value, log_ret);
        returns.push(log_ret);
        previous = md.value;
    }

    let mean = mean(&returns);
    let std = std_deviation(&returns);
    println!("Returns mean {} and std {} ", mean.unwrap(), std.unwrap());

    let std_val = std.unwrap();
    let labels = ["< 0.5 std", "< 1.0 std", "< 1.5 std", "< 2.0 std", "< 2.5 std", "< 3.0 std", ">= 3.0 std"];

    let mut counts = [0usize; 7];
    for &r in &returns {
        let abs_r = r.abs();
        if abs_r < 0.5 * std_val { counts[0] += 1; }
        if abs_r < 1.0 * std_val { counts[1] += 1; }
        if abs_r < 1.5 * std_val { counts[2] += 1; }
        if abs_r < 2.0 * std_val { counts[3] += 1; }
        if abs_r < 2.5 * std_val { counts[4] += 1; }

        if abs_r < 3.0 * std_val { counts[5] += 1; }
        counts[6] += 1;
    }

    let total = returns.len() as f64;
    println!("\nReturn distribution:");
    for i in 0..7 {
        println!("  {}: {} ({:.2}%)", labels[i], counts[i], counts[i] as f64 / total * 100.0);
    }

    /*let client = IbkrClient::new().expect("Failed to create IBKR client");

    let market_data = fetch_aapl_history(&client);
    println!("Fetched {} market data points", market_data.len());


    for md in market_data {
        marketdata_repo.create(md);
    }*/
}