use std::{io::Write, path::Path};
use chrono::{DateTime, Utc, Duration, Datelike, Weekday, Days};
use rand::{thread_rng, Rng};

mod cities;    use cities::*;
mod clients;   use clients::*;
mod products;  use products::*;
mod utils;     use utils::*;

#[derive(Debug)]
struct Sale {
    pub date: DateTime<Utc>,
    pub product: &'static str,
    pub client: &'static str,
    pub city: &'static str,
    pub price: f32,
    pub amount: u32
}

pub const DAY_SALES_RANGE: std::ops::Range<u32> = 0 .. 20;
pub const DISCOUNT_RANGE: std::ops::Range<f32> = 0.0 .. 1.0;
pub const PRICE_RANGE: std::ops::Range<f32> = 5.0 .. 50.0;
pub const PRICE_VARIATION_RANGE: std::ops::Range<f32> = -1.0 .. 1.0;
pub const AMOUNT_RANGE: std::ops::Range<u32> = 1 .. 100;
pub const SALES_FILE_NAME: &'static str = "sales";
pub const PRICE_HISTORY_FILE_NAME: &'static str = "price_history";

fn main() {
    let end_day = Utc::now();
    let start_day = end_day - Duration::days(365 * 4);
    let date_range = DateRange(start_day, end_day);
    let date_range_days = date_range.clone().count();
    
    // Price history
    println!("Gerando histórico de preços aleatoriamente...");
    
    let price_history_path = Path::new("../").join(PRICE_HISTORY_FILE_NAME).with_extension("csv");
    
    let mut price_history_file = std::fs::OpenOptions::new()
        .create(true).read(true).write(true).truncate(true)
        .open(&price_history_path).unwrap();
    
    write!(&mut price_history_file, "Data,").unwrap();
    let product_date_price = PRODUCTS.into_iter().map(|name| {
        let mut price = thread_rng().gen_range(PRICE_RANGE);
        write!(&mut price_history_file, "{name},").unwrap();
        (0..date_range_days).map(|_| {
            if thread_rng().gen_ratio(1, 15) {
                price = price - thread_rng().gen_range(PRICE_VARIATION_RANGE);
            }
            price
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    writeln!(&mut price_history_file).unwrap();

    for (date_id, date) in date_range.clone().enumerate() {
        write!(&mut price_history_file, "{},", format_date(date)).unwrap();
        for date_prices in product_date_price.iter() {
            write!(&mut price_history_file, "{:.2},", date_prices[date_id]).unwrap();
        }
        writeln!(&mut price_history_file).unwrap();
    }
    
    // Sales
    println!("Gerando tabela de vendas aleatoriamente...");

    let sales_path = Path::new("../").join(SALES_FILE_NAME).with_extension("csv");
    
    let mut sales_file = std::fs::OpenOptions::new()
        .create(true).read(true).write(true).truncate(true)
        .open(&sales_path).unwrap();

    writeln!(&mut sales_file, "Data,Produto,Cliente,Cidade,Valor venda un.,Quantidade").unwrap();

    for (day_id, day) in date_range.enumerate() {
        if let Weekday::Sun = day.weekday() {
            continue
        }
        for sale in
            (0..thread_rng().gen_range(DAY_SALES_RANGE)).into_iter()
            .map(|_| {
                let product_id = thread_rng().gen_range(0..PRODUCTS.len());
                let product_price_in_day = product_date_price[product_id][day_id];
                Sale {
                    date: day,
                    product: PRODUCTS[product_id],
                    client: CLIENTS[thread_rng().gen_range(0..CLIENTS.len())],
                    city: CITIES[thread_rng().gen_range(0..CITIES.len())],
                    price: product_price_in_day - thread_rng().gen_range(DISCOUNT_RANGE),
                    amount: thread_rng().gen_range(AMOUNT_RANGE)
                }
            })
        {
            let date = format_date(sale.date);
            let prod_name = sale.product;
            let cli = sale.client;
            let city = sale.city;
            let price = sale.price;
            let amount = sale.amount;
            writeln!(&mut sales_file, "{date},{prod_name},{cli},{city},{price:.2},{amount}").unwrap()
        }
    }

    println!("Processo finalizado, arquivos escritos com sucesso.");
}