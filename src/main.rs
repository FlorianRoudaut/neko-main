use std::fs;
use uuid::Uuid;
use prost::Message;
use neko_ibkr_api::client::IbkrClient;
use neko_ibkr_api::contract_stock::get_contract_stock_sync;
use neko_securities::{Stock, Key};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/neko.rs"));
}

fn stock_to_proto(s: &Stock) -> proto::Stock {
    proto::Stock {
        key: Some(proto::Key {
            id: s.key.id.to_string(),
            version: s.key.version,
            unique_name: s.key.unique_name.clone(),
        }),
        name: s.name.clone(),
        bbg_id: s.bbg_id.clone(),
        cusip: s.cusip.clone(),
        isin: s.isin.clone(),
        ibkr_contract_id: s.ibkr_contract_id.clone(),
        exchange_ids: s.exchange_ids.clone(),
    }
}

fn proto_to_stock(p: proto::Stock) -> Stock {
    let key = p.key.unwrap_or_default();
    Stock {
        key: Key {
            id: key.id.parse().unwrap_or_else(|_| Uuid::new_v4()),
            version: key.version,
            unique_name: key.unique_name,
        },
        name: p.name,
        bbg_id: p.bbg_id,
        cusip: p.cusip,
        isin: p.isin,
        ibkr_contract_id: p.ibkr_contract_id,
        exchange_ids: p.exchange_ids,
    }
}

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
    let client = IbkrClient::new().expect("Failed to create IBKR client");

    let tickers = ["AAPL", "MSFT", "GOOGL", "META"];
    let stocks: Vec<Stock> = tickers.iter().map(|t| fetch_stock(&client, t)).collect();

    let stock_list = proto::StockList {
        stocks: stocks.iter().map(stock_to_proto).collect(),
    };
    let blob = stock_list.encode_to_vec();
    fs::write("equities.bin", &blob).expect("Failed to write file");

    let bytes = fs::read("equities.bin").expect("Failed to read file");
    let loaded_list = proto::StockList::decode(bytes.as_slice()).expect("Deserialization failed");
    let loaded: Vec<Stock> = loaded_list.stocks.into_iter().map(proto_to_stock).collect();

    assert_eq!(loaded.len(), tickers.len());
    for (stock, ticker) in loaded.iter().zip(tickers.iter()) {
        assert_eq!(stock.name, *ticker);
        println!("ticker: {}, conid: {}", stock.name, stock.ibkr_contract_id);
    }

    println!("All {} stocks round-tripped successfully.", loaded.len());
}
