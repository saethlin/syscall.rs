// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! syscall {
    ($nr:ident) => {
        ::sc::syscall0(::sc::nr::$nr)
    };

    ($nr:ident, $a1:expr) => {
        ::sc::syscall1(::sc::nr::$nr, $a1)
    };

    ($nr:ident, $a1:expr, $a2:expr) => {
        ::sc::syscall2(::sc::nr::$nr, $a1, $a2)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        ::sc::syscall3(::sc::nr::$nr, $a1, $a2, $a3)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        ::sc::syscall4(::sc::nr::$nr, $a1, $a2, $a3, $a4)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        ::sc::syscall5(::sc::nr::$nr, $a1, $a2, $a3, $a4, $a5)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        ::sc::syscall6(::sc::nr::$nr, $a1, $a2, $a3, $a4, $a5, $a6)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr) => {
        ::sc::syscall7(::sc::nr::$nr, $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
}
