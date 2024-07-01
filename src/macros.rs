/*
 * macros.rs
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

/// Like `std::write!()`, except it asserts the writing succeeded.
///
/// This is done because the only failure mode for writing to a `String`
/// would be insufficient memory, which would cause an abort anyways.
///
/// # See also
/// * [`str_writeln!`](macro.str_writeln.html)
macro_rules! str_write {
    ($dest:expr, $($arg:tt)*) => {{
        use std::fmt::Write;
        write!($dest, $($arg)*).expect("Writing to string failed");
    }};
}

/// Like `std::writeln!()`, except it asserts the writing succeeded.
///
/// This is done because the only failure mode for writing to a `String`
/// would be insufficient memory, which would cause an abort anyways.
///
/// # See also
/// * [`str_write!`](macro.str_write.html)
macro_rules! str_writeln {
    ($dest:expr, $($arg:tt)*) => {{
        use std::fmt::Write;
        writeln!($dest, $($arg)*).expect("Writing to string failed");
    }};
}
