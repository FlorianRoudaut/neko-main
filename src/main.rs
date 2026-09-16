use std::fs;
use neko_ibkr_api::client::IbkrClient;
use neko_ibkr_api::contract_stock::get_contract_stock_sync;
use neko_securities::Equity;

fn fetch_equity(client: &IbkrClient, ticker: &str) -> Equity {
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

    Equity {
        ticker: ticker.to_string(),
        exchange: contract.exchange.clone(),
    }
}

fn main() {
    let client = IbkrClient::new().expect("Failed to create IBKR client");

    let tickers = ["AAPL", "MSFT", "GOOGL", "META"];
    let equities: Vec<Equity> = tickers.iter().map(|t| fetch_equity(&client, t)).collect();

    // Serialize and write to file
    let blob = bincode::serialize(&equities).expect("Serialization failed");
    fs::write("equities.bin", &blob).expect("Failed to write file");

    // Read and deserialize
    let bytes = fs::read("equities.bin").expect("Failed to read file");
    let loaded: Vec<Equity> = bincode::deserialize(&bytes).expect("Deserialization failed");

    // Verify
    assert_eq!(loaded.len(), tickers.len());
    for (equity, ticker) in loaded.iter().zip(tickers.iter()) {
        assert_eq!(equity.ticker, *ticker);
        println!("ticker: {}, exchange: {}", equity.ticker, equity.exchange);
    }

    println!("All {} equities round-tripped successfully.", loaded.len());
}
