// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 Andre Richter <andre.o.richter@gmail.com>

//! Null console.

use crate::synchronization::NullLock;

use super::interface::{self, ConsoleWrite};
use core::fmt;

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

pub struct SharedNullConsole(NullLock<NullConsole>);

pub struct NullConsole;

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

pub static mut NULL_CONSOLE: NullConsole = NullConsole;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

impl interface::ConsoleWrite for NullConsole {
    fn write_byte(&mut self, _c: u8) {}

    fn flush(&self) {}
}

impl core::fmt::Write for NullConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl interface::Read for NullConsole {
    fn clear_rx(&self) {}
}

impl interface::Statistics for NullConsole {}
impl interface::Console for NullConsole {}
