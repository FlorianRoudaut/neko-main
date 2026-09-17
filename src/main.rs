use std::fs;
use prost::Message;
use neko_ibkr_api::client::IbkrClient;
use neko_ibkr_api::contract_stock::get_contract_stock_sync;
use neko_securities::{Stock, Key};
use neko_securities::proto::{proto_to_stock, StockList};
use uuid::Uuid;

fn fetch_stock(client: &IbkrClient, ticker: &str) -> Stock {
    let result = get_contract_stock_sync(client, ticker)
        .unwrap_or_else(|e| panic!("Failed to fetch {} contract: {}", ticker, e));

    let stock_info = result
        .get(ticker)
        .and_then(|infos| infos.first())
        .unwrap_or_else(|| panic!("No info returned for {}", ticker));

    let contract = stock_info
        .contracts
        .iter()
        .find(|c| c.is_us)
        .unwrap_or_else(|| panic!("No US contract found for {}", ticker));

    Stock {
        key: Key { id: Uuid::new_v4(), version: 0, unique_name: ticker.to_string() },
        name: ticker.to_string(),
        bbg_id: String::new(),
        cusip: String::new(),
        isin: String::new(),
        ibkr_contract_id: contract.conid.to_string(),
        exchange_ids: vec![],
    }
}

fn main() {
    /*let client = IbkrClient::new().expect("Failed to create IBKR client");

    let tickers = ["AAPL", "MSFT", "GOOGL", "META"];
    let stocks: Vec<Stock> = tickers.iter().map(|t| fetch_stock(&client, t)).collect();

    let stock_list = StockList {
        stocks: stocks.iter().map(stock_to_proto).collect(),
    };
    let blob = stock_list.encode_to_vec();
    fs::write("equities.bin", &blob).expect("Failed to write file");*/

    let bytes = fs::read("equities.bin").expect("Failed to read file");
    let loaded_list = StockList::decode(bytes.as_slice()).expect("Deserialization failed");
    let loaded: Vec<Stock> = loaded_list.stocks.into_iter().map(proto_to_stock).collect();

    for stock in &loaded {
        println!("ticker: {}, conid: {}", stock.name, stock.ibkr_contract_id);
    }

    println!("Loaded {} stocks successfully.", loaded.len());
}
