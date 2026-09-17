use std::fs;
use uuid::Uuid;
use neko_ibkr_api::client::IbkrClient;
use neko_ibkr_api::contract_stock::get_contract_stock_sync;
use neko_securities::Stock;
use neko_securities::Key;

fn fetch_equity(client: &IbkrClient, ticker: &str) -> Stock {
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

    let key = Key { id: Uuid::new_v4() ,version:0, unique_name:ticker.to_string() };
    Stock {
        name: ticker.to_string(),
        exchange_ids: vec!(),
        bbg_id: String::new(),
        cusip: String::new(),
        ibkr_contract_id: contract.conid.to_string(),
        isin: String::new(),
        key: key,
    }
}

fn main() {
    let client = IbkrClient::new().expect("Failed to create IBKR client");

    let tickers = ["AAPL", "MSFT", "GOOGL", "META"];
    let equities: Vec<Stock> = tickers.iter().map(|t| fetch_equity(&client, t)).collect();

    // Serialize and write to file
    let blob = bincode::serialize(&equities).expect("Serialization failed");
    fs::write("equities.bin", &blob).expect("Failed to write file");

    // Read and deserialize
    let bytes = fs::read("equities.bin").expect("Failed to read file");
    let loaded: Vec<Stock> = bincode::deserialize(&bytes).expect("Deserialization failed");

    // Verify
    assert_eq!(loaded.len(), tickers.len());
    for (equity, ticker) in loaded.iter().zip(tickers.iter()) {
        assert_eq!(equity.name, *ticker);
        println!("ticker: {}, conid: {:?}", equity.name, equity.ibkr_contract_id);
    }

    println!("All {} equities round-tripped successfully.", loaded.len());
}
