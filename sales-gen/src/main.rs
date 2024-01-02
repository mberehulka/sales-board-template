use std::io::Write;
use chrono::{DateTime, Utc, Duration, Datelike, Weekday};

mod cities;    use cities::*;
mod clients;   use clients::*;
mod products;  use dirs::desktop_dir;
use products::*;
mod utils;     use rand::{thread_rng, Rng};
use utils::*;

#[derive(Debug)]
struct Sale<'s> {
    pub date: DateTime<Utc>,
    pub product: &'s Product,
    pub client: &'s Client,
    pub city: &'s City,
    pub price: f32,
    pub amount: u32
}

pub const DAY_SALES_RANGE: std::ops::Range<u32> = 0 .. 100;
pub const DISCOUNT_RANGE: std::ops::Range<f32> = 0.0 .. 5.0;
pub const AMOUNT_RANGE: std::ops::Range<u32> = 1 .. 100;
pub const FILE_NAME: &'static str = "sales";

fn main() {
    println!("Gerando tabela de vendas aleatória...");

    let path = desktop_dir().unwrap().join(FILE_NAME).with_extension("csv");
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(&path).unwrap();

    writeln!(&mut file, "Data,Produto,Valor produto,Cliente,Cidade,Valor venda un.,Quantidade").unwrap();

    for day in DateRange(
        Utc::now() - Duration::days(365 * 4),
        Utc::now()
    ) {
        if let Weekday::Sun = day.weekday() {
            continue
        }
        for sale in
            (0..thread_rng().gen_range(DAY_SALES_RANGE)).into_iter()
            .map(|_| {
                let product = Product::random();
                Sale {
                    date: day,
                    product,
                    client: Client::random(),
                    city: City::random(),
                    price: product.price - thread_rng().gen_range(DISCOUNT_RANGE),
                    amount: thread_rng().gen_range(AMOUNT_RANGE)
                }
            })
        {
            let date = sale.date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            let prod_name = sale.product.name;
            let prod_price = sale.product.price;
            let cli = sale.client.name;
            let city = sale.city.name;
            let price = sale.price;
            let amount = sale.amount;
            writeln!(&mut file,
                "{date},{prod_name},{prod_price:.2},{cli},{city},{price:.2},{amount}"
            ).unwrap()
        }
    }

    println!("Processo finalizado, arquivo salvo em: {path:?}.");
}