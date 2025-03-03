// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! System console.

mod null_console;

use crate::synchronization::{self, NullLock};

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    use core::fmt;

    pub trait SharedConsole {
        fn lock<R>(&self, f: impl FnOnce(&mut dyn ConsoleWrite) -> R) -> R;
    }

    /// Console write functions.
    pub trait ConsoleWrite: fmt::Write {
        /// Write a single character.
        fn write_byte(&mut self, c: u8);

        /// Block until the last buffered character has been physically put on the TX wire.
        #[allow(unused)]
        fn flush(&self);
    }

    /// Console read functions.
    pub trait Read {
        /// Read a single character.
        fn read_char(&self) -> u8 {
            ' ' as u8
        }

        /// Clear RX buffers, if any.
        fn clear_rx(&self);
    }

    /// Console statistics.
    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of characters read.
        #[allow(unused)]
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait Console: ConsoleWrite + Read + Statistics {}
}

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

pub static CUR_CONSOLE: NullLock<&'static mut (dyn interface::Console + Send)> =
    NullLock::new(unsafe { &mut null_console::NULL_CONSOLE });

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
use synchronization::interface::Mutex;

/// Register a new console.
pub fn register_console(new_console: &'static mut (dyn interface::Console + Send)) {
    CUR_CONSOLE.lock(|con| *con = new_console);
}
