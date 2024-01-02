use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Product {
    pub name: &'static str,
    pub price: f32
}
impl Product {
    pub const fn new(name: &'static str, price: f32) -> Self {
        Self { name, price }
    }
    pub fn random() -> &'static Self {
        &PRODUCTS[thread_rng().gen_range(0..PRODUCTS.len())]
    }
}
pub const PRODUCTS: &[Product] = &[
    Product::new("Picanha", 28.98),
    Product::new("Alcatra", 21.98),
    Product::new("Contra filé", 21.98),
    Product::new("Filé mingnon", 27.98),
    Product::new("Chandanga", 18.98),
    Product::new("Maminha", 21.98),
    Product::new("Lagarto", 17.98),
    Product::new("Cupim maturado", 26.98),
    Product::new("Peito bovino", 15.98),
    Product::new("Acém bovino", 15.98),
    Product::new("Paleta bovino", 15.98),
    Product::new("Pexinho bovino", 15.98),
    Product::new("Capa do coxão mole", 15.98),
    Product::new("Capa do contra filé", 15.98),
    Product::new("Costela Bovina", 8.98),
    Product::new("Lombo", 15.98),
    Product::new("Costelinha", 15.98),
    Product::new("Pernil com osso", 9.98),
    Product::new("Toucinho", 6.98),
];