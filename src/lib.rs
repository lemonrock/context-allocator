// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(arbitrary_self_types)]
#![feature(core_intrinsics)]
#![feature(thread_local)]


//! #context-allocator
//! 
//! This is a rust library.


#[macro_use] extern crate likely;


use self::binary_search_trees::*;
use self::binary_search_trees::red_black_tree::*;
use self::extensions::*;
use ::std::alloc::CannotReallocInPlace;
use ::std::alloc::Layout;
use ::std::alloc::GlobalAlloc;
use ::std::alloc::Alloc;
use ::std::alloc::AllocErr;
use ::std::alloc::Excess;
use ::std::collections::Bound;
use ::std::collections::Bound::*;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::cmp::max;
use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::num::NonZeroU32;
use ::std::num::NonZeroUsize;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;


include!("Allocator.rs");
include!("AllocatorAdaptor.rs");
include!("AllocatorState.rs");
include!("AllocToAllocatorAdaptor.rs");
include!("BumpAllocator.rs");
include!("CurrentAllocatorInUse.rs");
include!("GlobalAllocToAllocatorAdaptor.rs");
include!("GlobalThreadAndCoroutineSwitchableAllocator.rs");
include!("MemoryAddress.rs");
include!("MemoryRange.rs");
include!("MultipleBinarySearchTreeAllocator.rs");


pub(crate) mod binary_search_trees;


pub(crate) mod extensions;
