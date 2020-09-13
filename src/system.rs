use std::collections::HashMap;

use serde::Deserialize;

// TODO: parse all dates to chrono::DateTimes
// TODO: make further structs to split further based on subtypes
// TODO: should we normalize the data set?

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

// TODO: fill this out once you see one.
#[derive(Deserialize, Debug)]
pub struct Station {
    name: String,
    // some planetary outposts don't have controlling factions ref: Moskowitz's Progress
    controllingFaction: Option<String>,
    // fleet carriers I guess don't have controlling factions
    controllingFactionState: Option<String>,
    distanceToArrival: Option<f64>, // why is this an option? ref. Moskowitz's Progress
    primaryEconomy: Option<String>, // Why wouldn't a station have a primary economy?? ref. HIP 8593
    government: Option<String>,     // sometimes you don't get governments ref. Mead Port in Huave
    services: Vec<String>,
    r#type: String, // TODO: enum this up!
    id: u64,
    updateTime: String,
    market: Option<Market>,
    outfitting: Option<Outfitting>,
}

#[derive(Deserialize, Debug)]
pub struct Market {
    commodities: Vec<Commodity>,
    prohibitedCommodities: Vec<String>,
    updateTime: String,
}

#[derive(Deserialize, Debug)]
pub struct Commodity {
    name: String,
    symbol: String,
    category: String,
    commodityId: u64,
    demand: u64,
    supply: u64,
    buyPrice: u64,
    sellPrice: u64,
}

#[derive(Deserialize, Debug)]
pub struct Outfitting {
    updateTime: String,
    modules: Vec<Module>,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    name: String,
    symbol: String,
    moduleId: u64,
    class: u64,
    rating: String,   // TODO: enum
    category: String, // TODO: enum
}

// fields unique to stars.
#[derive(Deserialize, Debug)]
pub struct Star {
    #[serde(flatten)]
    common: BodyCommon,
    // whether this star is the main star in the system.
    // Why is this optional? I have no idea. TODO: figure out why
    mainStar: Option<bool>,
    // age of the star in years
    age: u64,
    // Black holes don't have a spectral class, but stars do, but both go under the type "Star"
    spectralClass: Option<String>,  // TODO: Replace with enum
    luminosity: String,             // TODO: replace with enum
    absoluteMagnitude: Option<f64>, // TODO: why is this optional?
    solarMasses: f64,
    solarRadius: f64,
}

// fields unique to planets
#[derive(Debug, Deserialize)]
pub struct Planet {
    #[serde(flatten)]
    common: BodyCommon,

    // TODO: should this default to false?
    isLandable: Option<bool>,
    gravity: f64,
    earthMasses: f64,
    radius: f64,
    surfacePressure: Option<f64>, // TODO: only present if there's an atmosphere
    // sometimes planets don't have volcanism
    volcanismType: Option<String>, // TODO: replace with enum
    // sometimes planets don't have atmospheres, hence the option.
    atmosphereType: Option<String>, // TODO: replace with enum
    // TODO: figure out what this is trying to convey and adjust deserialization appropriately.
    // Current best guess: a very tortured way to denote which star it's orbiting.
    // It can also be optional. Why?
    parents: Option<Vec<HashMap<String, u64>>>,
    // Material -> percentage
    atmosphereComposition: Option<HashMap<String, f64>>,
    // Material -> percentage
    solidComposition: Option<HashMap<String, f64>>,
    // gas giants can't be terraformed
    terraformingState: Option<String>, // TODO: replace with enum
}
