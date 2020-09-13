use serde::Deserialize;

use super::BodyCommon;

// fields unique to stars.
#[derive(Deserialize, Debug)]
pub struct Star {
    #[serde(flatten)]
    pub common: BodyCommon,
    // whether this star is the main star in the system.
    // Why is this optional? I have no idea. TODO: figure out why
    mainStar: Option<bool>,
    // age of the star in years
    age: u64,
    // what type of star this is
    #[serde(flatten)]
    pub r#type: StarType,
    // Black holes don't have a spectral class, but stars do, but both go under the type "Star"
    spectralClass: Option<String>,  // TODO: Replace with enum
    luminosity: String,             // TODO: replace with enum
    absoluteMagnitude: Option<f64>, // TODO: why is this optional?
    solarMasses: f64,
    solarRadius: f64,
}

#[derive(Deserialize, Debug)]
#[serde(tag="subType")]
pub enum StarType {
    #[serde(rename="M (Red giant) Star")]
    M_RedGiant,
    #[serde(rename="T (Brown dwarf) Star")]
    T_BrownDwarf,
    #[serde(rename="K (Yellow-Orange giant) Star")]
    K_Giant,
    #[serde(rename="White Dwarf (DB) Star")]
    DB,
    #[serde(rename="CJ Star")]
    CJ,
    #[serde(rename="C Star")]
    C,
    #[serde(rename="S-type Star")]
    S,
    #[serde(rename="White Dwarf (DAV) Star")]
    DAV,
    #[serde(rename="MS-type Star")]
    MS,
    #[serde(rename="Neutron Star")]
    Neutron,
    #[serde(rename="M (Red dwarf) Star")]
    M_RedDwarf,
    #[serde(rename="G (White-Yellow) Star")]
    G,
    #[serde(rename="Wolf-Rayet NC Star")]
    NC,
    #[serde(rename="White Dwarf (DCV) Star")]
    DCV,
    #[serde(rename="Herbig Ae/Be Star")]
    Herbig,
    #[serde(rename="Wolf-Rayet O Star")]
    O_WolfRayet,
    #[serde(rename="F (White) Star")]
    F,
    #[serde(rename="Wolf-Rayet C Star")]
    C_WolfRayet,
    #[serde(rename="White Dwarf (DAZ) Star")]
    DAZ,
    #[serde(rename="G (White-Yellow super giant) Star")]
    G_Supergiant,
    #[serde(rename="White Dwarf (DBV) Star")]
    DBV,
    #[serde(rename="T Tauri Star")]
    T_Tauri,
    #[serde(rename="White Dwarf (DC) Star")]
    DC,
    #[serde(rename="Supermassive Black Hole")]
    SuperMassive_BlackHole,
    #[serde(rename="CN Star")]
    CN,
    #[serde(rename="White Dwarf (DBZ) Star")]
    DBZ,
    #[serde(rename="L (Brown dwarf) Star")]
    L,
    #[serde(rename="White Dwarf (D) Star")]
    D,
    #[serde(rename="B (Blue-White super giant) Star")]
    B_Supergiant,
    #[serde(rename="Wolf-Rayet N Star")]
    N_WolfRayet,
    #[serde(rename="White Dwarf (DQ) Star")]
    DQ,
    #[serde(rename="Black Hole")]
    BlackHole,
    #[serde(rename="Wolf-Rayet Star")]
    WolfRayet,
    #[serde(rename="White Dwarf (DAB) Star")]
    DAB,
    #[serde(rename="White Dwarf (DA) Star")]
    DA,
    #[serde(rename="O (Blue-White) Star")]
    O,
    #[serde(rename="M (Red super giant) Star")]
    M_Supergiant,
    #[serde(rename="K (Yellow-Orange) Star")]
    K,
    #[serde(rename="B (Blue-White) Star")]
    B,
    #[serde(rename="Y (Brown dwarf) Star")]
    Y,
    #[serde(rename="A (Blue-White super giant) Star")]
    A_Supergiant,
    #[serde(rename="F (White super giant) Star")]
    F_Supergiant,
    #[serde(rename="A (Blue-White) Star")]
    A,
}