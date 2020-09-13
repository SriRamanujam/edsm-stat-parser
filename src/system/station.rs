use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Station<'a> {
    name: &'a str,
    // some planetary outposts don't have controlling factions ref: Moskowitz's Progress
    controllingFaction: Option<&'a str>,
    // fleet carriers I guess don't have controlling factions
    controllingFactionState: Option<&'a str>,
    distanceToArrival: Option<f64>, // why is this an option? ref. Moskowitz's Progress
    primaryEconomy: Option<&'a str>, // Why wouldn't a station have a primary economy?? ref. HIP 8593
    government: Option<&'a str>,     // sometimes you don't get governments ref. Mead Port in Huave
    services: Vec<&'a str>,
    r#type: &'a str, // TODO: enum this up!
    id: u64,
    updateTime: &'a str,
    market: Option<Market<'a>>,
    outfitting: Option<Outfitting<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Market<'a> {
    commodities: Vec<Commodity<'a>>,
    prohibitedCommodities: Vec<&'a str>,
    updateTime: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Commodity<'a> {
    name: &'a str,
    symbol: &'a str,
    category: &'a str,
    commodityId: u64,
    demand: u64,
    supply: u64,
    buyPrice: u64,
    sellPrice: u64,
}

#[derive(Deserialize, Debug)]
pub struct Outfitting<'a> {
    updateTime: &'a str,
    modules: Vec<Module<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Module<'a> {
    name: &'a str,
    symbol: &'a str,
    moduleId: u64,
    class: u64,
    rating: &'a str,   // TODO: enum
    category: &'a str, // TODO: enum
}
