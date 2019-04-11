// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(arbitrary_self_types)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(thread_local)]


//! # context-allocator
//! 
//! This provides allocators suitable for a number of use cases.
//!
//! All of these allocators implement the traits `::std::alloc::GlobalAlloc` and `::std::alloc::Alloc`, as we as a common base trait, `Allocator`.
//!
//! The most useful is a global allocator which allows switching between thread, coroutine and global (and thuse lockable) memory allocators, using the macro `global_thread_and_coroutine_switchable_allocator()`.
//!
//! Allocators provided include:-
//!
//! * `BumpAllocator`, a never-freeing bump allocator with slight optimization for reallocating the last allocation.
//! * `BitSetAllocator`, an allocator that uses a bit set of free blocks; uses 64-bit chunks to optimize searches.
//! * `MultipleBinarySearchTreeAllocator`, an efficient allocator which minimizes fragmentation by using multiple red-black trees of free blocks which are aggresively defragmented.
//! * `ContextAllocator`, a choice of either `BumpAllocator`, `BitSetAllocator` or `MultipleBinarySearchTreeAllocator`.
//! * `MemoryMapAllocator`, a mmap allocator with support for NUMA policies.
//! * `GlobalThreadAndCoroutineSwitchableAllocator`, suitable for replacing the global allocator and provides switchable allocators for global, thread local and context (coroutine) local needs.
//!
//! Allocators use a `MemorySource` to obtain and release memory.
//! Memory sources provided include:-
//!
//! * `MemoryMapAllocator`, useful for thread-local allocators as it can obtain memory from NUMA-local memory.
//! * `ArenaMemorySource`, an arena of fixed blocks which is itself backed by a memory source; this is useful as a source for the `BumpAllocator` and `BitSetAllocator` when used for contexts.
//!
//! Additionally a number of adaptors are provided:-
//!
//! * `AllocatorAdaptor`, an adaptor of `Allocator` to `GlobalAlloc` and `Alloc`; use it by calling `Allocator.adapt()`
//! * `GlobalAllocToAllocatorAdaptor`, an adaptor of `GlobalAlloc` to `Allocator`, useful for assigning a global allocator to `GlobalThreadAndCoroutineSwitchableAllocator`.
//! * `AllocToAllocatorAdaptor`, an adaptor of `Alloc` to `Allocator`.
//!
//! When using `GlobalThreadAndCoroutineSwitchableAllocator`, it is possible to save and restore the allocator state for the currently running context (coroutine).
//! It is also possible to create a lockless, fast thread-local allocator which make use of NUMA memory, unlike a conventional malloc.
//!
//!
//! ## Future
//!
//! * Investigate wrapping [Rampant Pixel's Memory Allocator](https://github.com/rampantpixels/rpmalloc).
//! * Investigate using DPDK's allocator.
//! * Investigate a B-tree backed allocator.
//! * Investigate a design that uses multiple doubly-linked 'free' lists of blocks; blocks can be variable in size but the free list is sorted
//! 	* Iteration over a particular free-list range may encountered blocks too small, or blocks so large they can be split up.
//! 	* This design is similar to that used by DPDK.
//! 	* To make the allocator multi-threaded, DPDK takes a spin lock on a particular 'heap', which is a set of free lists.
//! * Investigate an arena allocator for fixed-size blocks (suitable for holding ready-to-rumble context allocators, say).
//! * Investigate a fall-back over-size allocator for a thread-local allocator, which could use the `NumaMemoryMapAllocator` underneath.
//! * Investigate supporting over-size allocations in `MultipleBinarySearchTreeAllocator` by scanning the largest binary search tree for contiguous blocks.
//! * Investigate a persistent-memory backed allocator.
//! * Properly support excess allocations and Alloc's grow_in_place functions, but only if these are used by downstream collections.
//! * Investigate the use of the `BMI1` intrinsics `_blsi_u64` (extract lowest set bit), `_blsmsk_u64` and `_blsr_u64`.


extern crate either;
#[macro_use] extern crate likely;
#[cfg(unix)] extern crate libc;
#[cfg(any(target_os = "android", target_os = "linux"))] extern crate syscall_alt;


#[cfg(test)] use self::arena_memory_source::*;
use self::binary_search_trees::*;
use self::binary_search_trees::red_black_tree::*;
use self::bit_set::*;
use self::extensions::*;
use self::global::*;
#[cfg(test)] use self::mmap::*;
use self::mmap::numa::*;
use ::either::*;
#[cfg(unix)] use ::libc::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::syscall_alt::syscalls::Syscall;
use ::std::alloc::CannotReallocInPlace;
use ::std::alloc::Layout;
use ::std::alloc::GlobalAlloc;
use ::std::alloc::Alloc;
use ::std::alloc::AllocErr;
use ::std::alloc::Excess;
#[cfg(test)] use ::std::alloc::System;
use ::std::collections::Bound;
use ::std::collections::Bound::*;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::cmp::max;
use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::num::NonZeroU32;
use ::std::num::NonZeroUsize;
use ::std::ops::Add;
use ::std::ops::Deref;
use ::std::ops::Shr;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;


/// A memory source which uses an arena.
pub mod arena_memory_source;


pub(crate) mod binary_search_trees;


/// A bit set based allocator; allows reallocations, but requires a linear scan to find free blocks.
pub mod bit_set;


pub(crate) mod extensions;


/// Global, switchable allocator.
#[macro_use] pub mod global;


/// A memory map (mmap) based allocator with support for NUMA.
#[cfg(unix)]
pub mod mmap;


include!("alloc.rs");
include!("global_alloc.rs");


include!("Allocator.rs");
include!("AllocatorAdaptor.rs");
include!("AllocToAllocatorAdaptor.rs");
include!("BumpAllocator.rs");
include!("ContextAllocator.rs");
include!("GlobalAllocToAllocatorAdaptor.rs");
include!("MemoryAddress.rs");
include!("MemorySource.rs");
include!("MultipleBinarySearchTreeAllocator.rs");


#[cfg(test)] global_thread_and_coroutine_switchable_allocator!(MyGlobalAllocator, BumpAllocator<ArenaMemorySource<MemoryMapAllocator>>, MultipleBinarySearchTreeAllocator<MemoryMapAllocator>, GlobalAllocToAllocatorAdaptor<System>, GlobalAllocToAllocatorAdaptor(System));
