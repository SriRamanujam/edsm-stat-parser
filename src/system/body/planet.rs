use std::collections::HashMap;

use serde::Deserialize;

use super::BodyCommon;

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
