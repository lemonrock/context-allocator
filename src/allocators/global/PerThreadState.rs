// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


#[doc(hidden)]
#[allow(dead_code)]
pub struct PerThreadState<CoroutineLocalAllocator: LocalAllocator, ThreadLocalAllocator: LocalAllocator>
{
	pub current_allocator_in_use: CurrentAllocatorInUse,

	pub coroutine_local_allocator: Option<CoroutineLocalAllocator>,

	pub thread_local_allocator: Option<ThreadLocalAllocator>,
}

impl<CoroutineLocalAllocator: LocalAllocator, ThreadLocalAllocator: LocalAllocator> PerThreadState<CoroutineLocalAllocator, ThreadLocalAllocator>
{
	#[doc(hidden)]
	#[inline(always)]
	pub const fn empty() -> Self
	{
		Self
		{
			current_allocator_in_use: CurrentAllocatorInUse::Global,
			coroutine_local_allocator: None,
			thread_local_allocator: None,
		}
	}
}
