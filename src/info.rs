/*
 * info.rs
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

mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use once_cell::sync::Lazy;

#[allow(unused_imports)]
pub use self::build::{
    GIT_COMMIT_HASH, NUM_JOBS, PKG_AUTHORS, PKG_DESCRIPTION, PKG_LICENSE, PKG_NAME, PKG_REPOSITORY,
    PKG_VERSION, RUSTC_VERSION, TARGET,
};

pub static VERSION_INFO: Lazy<String> = Lazy::new(|| {
    let mut version = format!("v{PKG_VERSION}");

    if let Some(commit_hash) = *GIT_COMMIT_HASH_SHORT {
        str_write!(&mut version, " [{commit_hash}]");
    }

    version
});

pub static FULL_VERSION: Lazy<String> = Lazy::new(|| {
    let mut version = format!("{PKG_NAME} {}\n\nCompiled:\n", *VERSION_INFO);

    str_writeln!(&mut version, "* across {NUM_JOBS} threads");
    str_writeln!(&mut version, "* by {RUSTC_VERSION}");
    str_writeln!(&mut version, "* for {TARGET}");

    version
});

pub static VERSION: Lazy<String> = Lazy::new(|| format!("{PKG_NAME} {}", *VERSION_INFO));

pub static GIT_COMMIT_HASH_SHORT: Lazy<Option<&'static str>> =
    Lazy::new(|| build::GIT_COMMIT_HASH.map(|s| &s[..8]));

#[test]
fn info() {
    assert!(VERSION.starts_with(PKG_NAME));
    assert!(VERSION.ends_with(&*VERSION_INFO));

    if let Some(hash) = *GIT_COMMIT_HASH_SHORT {
        assert_eq!(hash.len(), 8);
    }
}
