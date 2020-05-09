// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


use super::*;


pub(crate) mod binary_search_trees;


/// A bit set based allocator; allows reallocations, but requires a linear scan to find free blocks.
pub mod bit_set;


/// Global, switchable allocator.
#[macro_use] pub mod global;


include!("Allocator.rs");
include!("BumpAllocator.rs");
include!("ContextAllocator.rs");
include!("MultipleBinarySearchTreeAllocator.rs");
