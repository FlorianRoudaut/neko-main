use std::path::Path;

use neko_ibkr_api::{client::IbkrClient, marketdata_history::{HistoryRequest, get_marketdata_history_sync}};
use neko_marketdata::{MarketData, Metric, Source};
use neko_securities::Key;
use neko_tech_localstorage::LocalFileRepository;
use neko_tech_persistence::Repository;
use uuid::Uuid;

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

pub fn fetch_and_save_marketdata_history(client: &IbkrClient, ibkr_contract_id: u64) -> Vec<MarketData> {
    let data_dir = Path::new("../data");

    let source_repo: LocalFileRepository<Source> = LocalFileRepository::<Source>::load(data_dir)
        .expect("Failed to load source repository");
    let metric_repo: LocalFileRepository<Metric> = LocalFileRepository::<Metric>::load(data_dir)
        .expect("Failed to load metric repository");
    /*let mut stock_repo: LocalFileRepository<Stock> = LocalFileRepository::<Stock>::load(data_dir)
        .expect("Failed to load security repository");*/
    let stock_id = uuid::Uuid::new_v4();

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
        conid: ibkr_contract_id,
        period: "1y",
        bar: "1d",
        outside_rth: false,
        start_time: Some("20270101-00:00:00"),
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
                security_id: stock_id,
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