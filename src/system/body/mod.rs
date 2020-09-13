pub mod planet;
mod star;

use planet::Planet;
use serde::Deserialize;
use star::Star;

use super::station::Station;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Body<'a> {
    #[serde(borrow)]
    Star(Star<'a>),
    #[serde(borrow)]
    Planet(Planet<'a>),
}

// common fields shared between all types of bodies.
#[derive(Deserialize, Debug)]
pub struct BodyCommon<'a> {
    pub id64: u64,
    pub bodyId: u16,
    pub name: &'a str,
    pub distanceToArrival: f64,
    pub surfaceTemperature: Option<f64>, // TODO: this can be optional sometimes I suppose.
    pub orbitalPeriod: Option<f64>,      // TODO: this doesn't apply to black holes, I guess.
    pub semiMajorAxis: Option<f64>,      // TODO: this doesn't apply to black holes, I guess.
    pub orbitalEccentricity: Option<f64>, // TODO: this doesn't apply to black holes, I guess.
    pub orbitalInclination: Option<f64>, // TODO: this doesn't apply to block holes, I guess.
    pub argOfPeriapsis: Option<f64>,     // TODO: this doesn't apply to black holes, I guess.
    pub rotationalPeriod: Option<f64>,   // TODO: why would this be optional?
    pub rotationalPeriodTidallyLocked: Option<bool>, // TODO: this doesn't apply to black holes, I guess.
    pub axialTilt: Option<f64>, // TODO: this doesn't apply to black holes, I guess.
    pub stations: Vec<Station<'a>>,
    pub updateTime: &'a str,
}
