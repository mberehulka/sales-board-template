use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Client {
    pub name: &'static str
}
impl Client {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
    pub fn random() -> &'static Self {
        &CLIENTS[thread_rng().gen_range(0..CLIENTS.len())]
    }
}
pub const CLIENTS: &[Client] = &[
    Client::new("Limner Studio"),
    Client::new("Life Paths"),
    Client::new("PrimeHouse"),
    Client::new("Find Agenda"),
    Client::new("Et Cetera Systems"),
    Client::new("Handy Help"),
    Client::new("Puzzles"),
    Client::new("DualLight"),
    Client::new("Radiate Random"),
    Client::new("Resource Refresh"),
    Client::new("XyloFurniture"),
    Client::new("TRUE Random"),
    Client::new("Light Random"),
    Client::new("FirstVictory"),
    Client::new("Node Tech"),
    Client::new("Odds And Ends"),
    Client::new("Odd And Ends"),
    Client::new("Adventures"),
    Client::new("Forel Library"),
    Client::new("Silence Center"),
    Client::new("Authority Random"),
    Client::new("Spiritual Beings"),
    Client::new("Random Row"),
    Client::new("X Marks The Spot"),
    Client::new("Spray Shop"),
    Client::new("Cellar Random"),
    Client::new("Pop-Culture"),
    Client::new("In A Pinch"),
    Client::new("Et Cetera"),
    Client::new("Missed Opps"),
    Client::new("CraftCreate"),
    Client::new("Empty Particles"),
    Client::new("Though Random"),
    Client::new("Essentials"),
    Client::new("Airhead"),
    Client::new("Motion Random"),
    Client::new("Sporadic Systems"),
    Client::new("FursKips"),
    Client::new("Resource Random"),
    Client::new("Muno Random"),
    Client::new("Random Riders"),
    Client::new("23Rd Century"),
    Client::new("Destroy Random"),
    Client::new("Vappa Shop"),
    Client::new("Random Repair"),
    Client::new("Fleet Random"),
    Client::new("Knick Knacks"),
    Client::new("Absent-Minded"),
    Client::new("Random Reason"),
    Client::new("Eclipse Random"),
    Client::new("The Source"),
    Client::new("Cowboy Random"),
    Client::new("Million Random"),
    Client::new("Herb Random"),
    Client::new("Nick Knacks"),
    Client::new("Sense Random"),
    Client::new("Random Refresh"),
    Client::new("Lifestyle Random"),
    Client::new("Shady Bootz"),
];