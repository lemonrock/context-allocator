// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


use super::*;
use static_assertions::_core::marker::PhantomData;
use std::mem::replace;
use magic_ring_buffer::memory_sizes::MemorySize;


include!("CurrentAllocatorInUse.rs");
include!("GlobalThreadAndCoroutineSwitchableAllocator.rs");
include!("GlobalThreadAndCoroutineSwitchableAllocatorInstance.rs");
include!("LifetimeHint.rs");
include!("LocalAllocator.rs");
include!("MemoryRange.rs");
include!("PerThreadState.rs");
