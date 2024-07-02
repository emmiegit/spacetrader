/*
 * dynamic_enum.rs
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

use std::rc::Rc;

/// Dynamic enum
///
/// Takes a list of things from the game data file and then makes
/// a cheap enum type that can be use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicEnum {
    pub name: &'static str,
    pub values: Vec<String>,
}

impl DynamicEnum {
    pub fn new<I, S>(name: &'static str, iterator: I) -> Rc<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Rc::new(DynamicEnum {
            name,
            values: iterator.into_iter().map(|s| s.into()).collect(),
        })
    }

    pub fn value(self: &Rc<DynamicEnum>, value: &str) -> DynamicEnumValue {
        let parent = DynamicEnum::clone(self);
        let index = self.values.iter().position(|s| s == value);
        match index {
            None => panic!("Cannot find enum value '{}' in {}", value, self.name),
            Some(index) => DynamicEnumValue {
                parent: Rc::clone(self),
                index,
            },
        }
    }
}

/// Dynamic enum value
///
/// Represents a particular value within a dynamic enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicEnumValue {
    parent: Rc<DynamicEnum>,
    index: usize,
}

impl DynamicEnumValue {
    #[inline]
    pub fn value(&self) -> &str {
        &self.parent.values[self.index]
    }
}
