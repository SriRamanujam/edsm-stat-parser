use serde::Deserialize;

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

