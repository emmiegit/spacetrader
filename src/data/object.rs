/*
 * data/object.rs
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

use super::DataFile;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GameData {
    pub raw_toml: String,
    pub raw_toml_path: PathBuf,
    // TODO
}

impl GameData {
    pub fn load(raw_toml_path: PathBuf) -> Result<Self> {
        let (object, raw_toml) = DataFile::load(&raw_toml_path)?;
        todo!();

        Ok(GameData {
            raw_toml,
            raw_toml_path,
        })
    }
}