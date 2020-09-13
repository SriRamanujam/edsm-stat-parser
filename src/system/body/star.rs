use serde::Deserialize;

use super::BodyCommon;

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

