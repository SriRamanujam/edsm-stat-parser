mod star;
mod planet;

use serde::Deserialize;
use planet::Planet;
use star::Star;

use super::station::Station;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Body {
    Star(Star),
    Planet(Planet),
}

// common fields shared between all types of bodies.
#[derive(Deserialize, Debug)]
pub struct BodyCommon {
    id64: u64,
    bodyId: u16,
    name: String,
    subType: String,
    distanceToArrival: f64,
    surfaceTemperature: Option<f64>, // TODO: this can be optional sometimes I suppose.
    orbitalPeriod: Option<f64>,      // TODO: this doesn't apply to black holes, I guess.
    semiMajorAxis: Option<f64>,      // TODO: this doesn't apply to black holes, I guess.
    orbitalEccentricity: Option<f64>, // TODO: this doesn't apply to black holes, I guess.
    orbitalInclination: Option<f64>, // TODO: this doesn't apply to block holes, I guess.
    argOfPeriapsis: Option<f64>,     // TODO: this doesn't apply to black holes, I guess.
    rotationalPeriod: Option<f64>,   // TODO: why would this be optional?
    rotationalPeriodTidallyLocked: Option<bool>, // TODO: this doesn't apply to black holes, I guess.
    axialTilt: Option<f64>, // TODO: this doesn't apply to black holes, I guess.
    stations: Vec<Station>,
    updateTime: String,
}
