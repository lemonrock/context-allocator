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
#![feature(const_fn_fn_ptr_basics)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(nonzero_is_power_of_two)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(thread_local)]


//! # context-allocator
//! 
//! This provides allocators suitable for a number of use cases.
//!
//! All of these allocators implement the traits `std::alloc::GlobalAlloc` and `std::alloc::Alloc`, as we as a common base trait, `Allocator`.
//!
//! The most useful is a global allocator which allows switching between thread, coroutine and global (and thuse lockable) memory allocators, using the macro `global_thread_and_coroutine_switchable_allocator()`.
//!
//! Allocators provided include:-
//!
//! * `BumpAllocator`, a never-freeing bump allocator with slight optimization for reallocating the last allocation.
//! * `BitSetAllocator`, an allocator that uses a bit set of free blocks; uses 64-bit chunks to optimize searches.
//! * `MultipleBinarySearchTreeAllocator`, an efficient allocator which minimizes fragmentation by using multiple red-black trees of free blocks which are aggresively defragmented.
//! * `ContextAllocator`, a choice of either `BumpAllocator`, `BitSetAllocator` or `MultipleBinarySearchTreeAllocator`.
//! * `MemoryMapAllocator`, a NUMA-aware mmap allocator with support for NUMA policies.
//! * `GlobalThreadAndCoroutineSwitchableAllocator`, suitable for replacing the global allocator and provides switchable allocators for global, thread local and context (coroutine) local needs; must be created using the macro `global_thread_and_coroutine_switchable_allocator`.
//!
//! Allocators use a `MemorySource` to obtain and release memory.
//! Memory sources provided include:-
//!
//! * `MemoryMapSource`, useful for thread-local allocators as it can obtain memory from NUMA-local memory.
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
//! * Investigate a B-tree backed allocator.
//! * Investigate a design that uses multiple doubly-linked 'free' lists of blocks; blocks can be variable in size but the free list is sorted
//! 	* Iteration over a particular free-list range may encountered blocks too small, or blocks so large they can be split up.
//! 	* This design is similar to that used by DPDK.
//! 	* To make the allocator multi-threaded, DPDK takes a spin lock on a particular 'heap', which is a set of free lists.
//! * Investigate a fall-back over-size allocator for a thread-local allocator, which could use the `NumaMemoryMapSource` underneath.
//! * Investigate supporting over-size allocations in `MultipleBinarySearchTreeAllocator` by scanning the largest binary search tree for contiguous blocks.
//! * Investigate a persistent-memory backed allocator.
//! * Properly support excess allocations and Alloc's grow_in_place functions, but only if these are used by downstream collections.
//! * Investigate the use of the `BMI1` intrinsics `_blsi_u64` (extract lowest set bit), `_blsmsk_u64` and `_blsr_u64`.
//!
//!
//! ## Licensing
//!
//! The license for this project is MIT.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::adaptors::*;
use self::allocators::*;
use self::binary_search_trees::*;
use self::binary_search_trees::red_black_tree::*;
use self::extensions::*;
use self::memory_sources::*;
use either::*;
use likely::*;
use linux_support::memory::mapping::*;
use magic_ring_buffer::memory_sizes::MemorySize;
use std::alloc::Allocator as Alloc;
use std::alloc::AllocError;
use std::alloc::Layout;
use std::alloc::GlobalAlloc;
use std::alloc::System;
use std::collections::Bound;
use std::collections::Bound::*;
use std::cell::Cell;
use std::cell::UnsafeCell;
use std::cmp::max;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem::align_of;
use std::mem::ManuallyDrop;
use std::mem::replace;
use std::mem::size_of;
use std::mem::transmute;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
use std::ops::Add;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Shr;
use std::ops::Sub;
use std::ops::SubAssign;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::panic::RefUnwindSafe;
use std::panic::resume_unwind;
use std::panic::UnwindSafe;
use std::ptr::drop_in_place;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;
use std::sync::Arc;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::non_zero::new_non_zero_u32;
use swiss_army_knife::non_zero::new_non_zero_usize;


/// Adapt various allocator traits to one another.
pub mod adaptors;


/// Allocators.
pub mod allocators;


/// Extensions useful for working with memory; not a stable part of the API of this crate.
pub mod extensions;


/// Memory sources.
pub mod memory_sources;


include!("CurrentAllocatorInUse.rs");
include!("GloballyAllocated.rs");
include!("GlobalThreadAndCoroutineSwitchableAllocator.rs");
include!("GlobalThreadAndCoroutineSwitchableAllocatorInstance.rs");
include!("LifetimeHint.rs");
include!("LocalAllocator.rs");
include!("LocalAllocatorMemoryUsage.rs");
include!("MemoryAddress.rs");
include!("MemoryRange.rs");
include!("PerThreadState.rs");


#[cfg(test)] global_thread_and_coroutine_switchable_allocator!(MyGlobalAllocator, BumpAllocator<ArenaMemorySource<MemoryMapSource>>, MultipleBinarySearchTreeAllocator<MemoryMapSource>, GlobalAllocToAllocatorAdaptor<System>, GlobalAllocToAllocatorAdaptor(System));
