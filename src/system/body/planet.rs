use std::collections::HashMap;

use serde::Deserialize;

use super::BodyCommon;

// fields unique to planets
#[derive(Debug, Deserialize)]
pub struct Planet {
    #[serde(flatten)]
    pub common: BodyCommon,

    #[serde(flatten)]
    pub r#type: PlanetType,

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

#[derive(Debug, Deserialize)]
#[serde(tag="subType")]
pub enum PlanetType {
    #[serde(rename="Class I gas giant")]
    GasGiant_I,
    #[serde(rename="Class II gas giant")]
    GasGiant_II,
    #[serde(rename="Class III gas giant")]
    GasGiant_III,
    #[serde(rename="Class IV gas giant")]
    GasGiant_IV,
    #[serde(rename="Class V gas giant")]
    GasGiant_V,
    #[serde(rename="Gas giant with ammonia-based life")]
    GasGiant_AmmoniaLife,
    #[serde(rename="Gas giant with water-based life")]
    GasGiant_WaterLife,
    #[serde(rename="Helium-rich gas giant")]
    GasGiant_HeliumRich,
    #[serde(rename="Helium gas giant")]
    GasGiant_Helium,
    #[serde(rename="Water world")]
    WaterWorld,
    #[serde(rename="Metal-rich body")]
    MetalRichBody,
    #[serde(rename="Ammonia world")]
    AmmoniaWorld,
    #[serde(rename="High metal content world")]
    HighMetalContentWorld,
    #[serde(rename="Rocky body")]
    RockyBody,
    #[serde(rename="Earth-like world")]
    EarthLikeWorld,
    #[serde(rename="Icy body")]
    IcyBody,
    #[serde(rename="Rocky Ice world")]
    RockyIceWorld,
    #[serde(rename="Water giant")]
    WaterGiant,
}