use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

#[derive(Debug, Deserialize)]
struct Bitcoin;

#[derive(Debug, Deserialize)]
struct Ethereum;

#[derive(Debug, Deserialize)]
struct SP500;

trait Asset {
    fn fetch_price(&self) -> f64;
    fn record_price(&self, file_name: &str, price: f64);
}

impl Asset for Bitcoin {
    fn fetch_price(&self) -> f64 {
        let url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        let response = ureq::get(url).call().unwrap();
        let json: serde_json::Value = serde_json::from_reader(response.into_reader()).unwrap();
        json["bpi"]["USD"]["rate_float"].as_f64().unwrap()
    }

    fn record_price(&self, file_name: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(file_name).unwrap();
        writeln!(file, "Bitcoin: ${:.2}", price).unwrap();
    }
}

impl Asset for Ethereum {
    fn fetch_price(&self) -> f64 {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url).call().unwrap();
        let json: serde_json::Value = serde_json::from_reader(response.into_reader()).unwrap();
        json["ethereum"]["usd"].as_f64().unwrap()
    }

    fn record_price(&self, file_name: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(file_name).unwrap();
        writeln!(file, "Ethereum: ${:.2}", price).unwrap();
    }
}

impl Asset for SP500 {
    fn fetch_price(&self) -> f64 {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        let response = ureq::get(url).call().unwrap();
        let json: serde_json::Value = serde_json::from_reader(response.into_reader()).unwrap();
        json["chart"]["result"][0]["indicators"]["quote"][0]["close"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .as_f64()
            .unwrap()
    }

    fn record_price(&self, file_name: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(file_name).unwrap();
        writeln!(file, "S&P 500: ${:.2}", price).unwrap();
    }
}

fn main() {
    let bitcoin = Bitcoin;
    let ethereum = Ethereum;
    let sp500 = SP500;

    let assets = vec![
        ("Bitcoin", "bitcoin_prices.txt", &bitcoin as &dyn Asset),
        ("Ethereum", "ethereum_prices.txt", &ethereum as &dyn Asset),
        ("S&P 500", "sp500_prices.txt", &sp500 as &dyn Asset),
    ];

    loop {
        for (name, file_name, asset) in &assets {
            let price = asset.fetch_price();
            asset.record_price(file_name, price);
            println!("{} price saved: ${:.2}", name, price);
        }
        thread::sleep(Duration::from_secs(10));
    }
}
