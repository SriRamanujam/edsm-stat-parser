pub mod body;
pub mod station;

use body::Body;
use serde::Deserialize;
use station::Station;

#[derive(Deserialize, Debug)]
pub struct System<'a> {
    pub id64: u64,
    pub name: &'a str,
    pub coords: Coords,
    pub date: &'a str, // TODO: parse to a chrono::DateTime
    pub bodies: Vec<Body<'a>>,
    pub stations: Vec<Station<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

impl Coords {
    pub fn abs(&self) -> f64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
