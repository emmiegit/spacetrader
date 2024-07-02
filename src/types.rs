/*
 * types.rs
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

use currency_rs::{Currency, CurrencyOpts};
use once_cell::sync::Lazy;

// Type for money, two digits.

pub const MONEY_OPTS: Lazy<Option<CurrencyOpts>> = Lazy::new(|| {
    Some(
        CurrencyOpts::new()
            .set_symbol("$")
            .set_precision(2)
            .set_increment(0.01),
    )
});

pub type Money = Currency;

#[inline]
pub fn money(value: f64) -> Money {
    Currency::new_float(value, MONEY_OPTS.clone())
}

// Type for storage, one digit.

pub const STORAGE_OPTS: Lazy<Option<CurrencyOpts>> = Lazy::new(|| {
    Some(
        CurrencyOpts::new()
            .set_symbol("")
            .set_precision(2)
            .set_increment(0.1),
    )
});

pub type Storage = Currency;

#[inline]
pub fn storage(value: f64) -> Storage {
    Currency::new_float(value, STORAGE_OPTS.clone())
}
