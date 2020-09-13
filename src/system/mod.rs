
pub mod station;
pub mod body;

use body::Body;
use serde::Deserialize;
use station::Station;

#[derive(Deserialize, Debug)]
pub struct System {
    pub id64: u64,
    pub name: String,
    pub coords: Coords,
    pub date: String, // TODO: parse to a chrono::DateTime
    pub bodies: Vec<Body>,
    pub stations: Vec<Station>,
}

#[derive(Deserialize, Debug)]
pub struct Coords {
    x: f64,
    y: f64,
    z: f64,
}
