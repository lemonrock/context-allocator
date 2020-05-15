// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


use super::*;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::memory::mapping::*;
use magic_ring_buffer::*;


include!("CoroutineHeapMemory.rs");
include!("CoroutineHeapMemorySource.rs");
include!("CoroutineStackMemory.rs");
include!("MemorySource.rs");
include!("MemoryMapSource.rs");
