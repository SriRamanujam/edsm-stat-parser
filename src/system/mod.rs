
mod station;
mod body;

use body::Body;
use serde::Deserialize;
use station::Station;

#[derive(Deserialize, Debug)]
pub struct System {
    id64: u64,
    name: String,
    coords: Coords,
    date: String, // TODO: parse to a chrono::DateTime
    bodies: Vec<Body>,
    stations: Vec<Station>,
}

#[derive(Deserialize, Debug)]
pub struct Coords {
    x: f64,
    y: f64,
    z: f64,
}
