/*
 * data/file.rs
 *
 * mspacetrader - Midnight Space Trader
 * Copyright (C) 2024 Emmie Smith
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the “Software”), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 */

use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DataFile {
    language: String,
    game: GameSection,
    goods: Vec<GoodSection>,
    planets: Vec<PlanetSection>,
    events: EventsSection,
    // TODO
}

impl DataFile {
    pub fn load(path: &Path) -> Result<(Self, String)> {
        // Read TOML
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse and build object
        let config = toml::from_str(&contents)?;
        Ok((config, contents))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct GameSection {
    starting_planet: String,
    starting_ship: String,
    base_consumption: HashMap<String, f32>,
    money: GameMoneySection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct GameMoneySection {
    starting_cash: f64,
    starting_debt: f64,
    starting_bank: f64,
    debt_interest: f32,
    bank_interest: f32,
    debt_limit: f64,
    debt_death: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct GoodSection {
    id: String,
    name: String,
    word: String,
    description: String,

    #[serde(default)]
    fractional: bool,

    #[serde(default)]
    illegal: bool,

    #[serde(default)]
    limit: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct PlanetSection {
    #[serde(default = "t")]
    enabled: bool,

    id: String,
    name: String,
    description: String,
    symbol: String,
    color: String,
    events: Vec<String>,
    features: Vec<String>,
    goods: Vec<PlanetGoodSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct PlanetGoodSection {
    id: String,
    stock: u32,
    regen: u32,
    min: u32,
    max: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct EventsSection {
    flavor: Vec<FlavorEventSection>,
    price: Vec<PriceEventSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct FlavorEventSection {
    description: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct PriceEventSection {
    id: String,
    goods: Vec<String>,
    description: Vec<String>,
    duration: (u32, u32),
    price: (f32, f32),
}

#[inline]
fn t() -> bool {
    true
}
