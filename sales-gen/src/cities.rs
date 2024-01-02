use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct City {
    pub name: &'static str
}
impl City {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
    pub fn random() -> &'static Self {
        &CITIES[thread_rng().gen_range(0..CITIES.len())]
    }
}
pub const CITIES: &[City] = &[
    City::new("Campo Grande"),
    City::new("Juazeiro"),
    City::new("Goiânia"),
    City::new("Maceió"),
    City::new("Belém"),
    City::new("Belo Horizonte"),
    City::new("Porto Alegre"),
    City::new("Salvador"),
    City::new("Vila Velha"),
];