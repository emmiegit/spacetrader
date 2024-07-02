/*
 * args.rs
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

use crate::data::GameData;
use crate::info;
use clap::{value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;
use std::process;

pub fn parse_args() -> GameData {
    let mut matches = Command::new("DEEPWELL")
        .author(info::PKG_AUTHORS)
        .version(info::VERSION.as_str())
        .long_version(info::FULL_VERSION.as_str())
        .about(info::PKG_DESCRIPTION)
        .arg(
            Arg::new("data-file")
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Set)
                .required(true)
                .help("The game data file to use"),
        )
        .get_matches();

    let data_path = matches
        .remove_one::<PathBuf>("data-file")
        .expect("Required argument not provided");

    match GameData::load(data_path) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Unable to load configuration from file: {error}");
            process::exit(1);
        }
    }
}
