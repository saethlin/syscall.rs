// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 Linux.

use crate::SyscallArgument;
use core::arch::asm;

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> *mut () {
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall1<A1>(n: usize, a1: A1) -> *mut ()
where
    A1: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall2<A1, A2>(n: usize, a1: A1, a2: A2) -> *mut ()
where
    A1: SyscallArgument,
    A2: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        in("rsi") a2.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall3<A1, A2, A3>(n: usize, a1: A1, a2: A2, a3: A3) -> *mut ()
where
    A1: SyscallArgument,
    A2: SyscallArgument,
    A3: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        in("rsi") a2.ptr(),
        in("rdx") a3.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall4<A1, A2, A3, A4>(n: usize, a1: A1, a2: A2, a3: A3, a4: A4) -> *mut ()
where
    A1: SyscallArgument,
    A2: SyscallArgument,
    A3: SyscallArgument,
    A4: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        in("rsi") a2.ptr(),
        in("rdx") a3.ptr(),
        in("r10") a4.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall5<A1, A2, A3, A4, A5>(
    n: usize,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
) -> *mut ()
where
    A1: SyscallArgument,
    A2: SyscallArgument,
    A3: SyscallArgument,
    A4: SyscallArgument,
    A5: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        in("rsi") a2.ptr(),
        in("rdx") a3.ptr(),
        in("r10") a4.ptr(),
        in("r8") a5.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall6<A1, A2, A3, A4, A5, A6>(
    n: usize,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    a6: A6,
) -> *mut ()
where
    A1: SyscallArgument,
    A2: SyscallArgument,
    A3: SyscallArgument,
    A4: SyscallArgument,
    A5: SyscallArgument,
    A6: SyscallArgument,
{
    let mut n = n.ptr();
    asm!(
        "syscall",
        inout("rax") n,
        in("rdi") a1.ptr(),
        in("rsi") a2.ptr(),
        in("rdx") a3.ptr(),
        in("r10") a4.ptr(),
        in("r8") a5.ptr(),
        in("r9") a6.ptr(),
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}
