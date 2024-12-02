#[derive(debug)]
struct Bitcoin {
    api_address:String,
    file_name:String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        return 32.0;
    }

    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}

struct Etherium {
    api_address:String,
    file_name:String,
}

struct SP500 {
    api_address:String;
    file_name:String;
};

fn main() {
    let btc_api = "bitcoin link".to_string();
    let btc_txt  = "btc_prices.json".to_string();
    let b: Bitcoin = Bitcoin{api_address:btc_api, file_name:btc_txt};

    print!()
}