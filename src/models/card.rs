#[derive(Default)]
#[derive(Debug)]

pub struct Card {
    pub id: usize,
    pub name: String,
    pub code: String,
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub speed: u8,
    pub killer_instinct: u8
}