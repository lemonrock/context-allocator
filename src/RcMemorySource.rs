// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Represents a Reference-counted (RC) memory source.
///
/// Useful when passing in a memory source which does not implement `Clone` to an allocator.
#[derive(Debug, Clone)]
pub struct RcMemorySource<MS: MemorySource>(Rc<MS>);

impl<MS: MemorySource> MemorySource for RcMemorySource<MS>
{
	#[inline(always)]
	fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		self.0.obtain(non_zero_size)
	}

	#[inline(always)]
	fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.0.release(non_zero_size, current_memory)
	}
}

impl<MS: MemorySource> Deref for RcMemorySource<MS>
{
	type Target = MS;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<MS: MemorySource> RcMemorySource<MS>
{
	/// Creates a new thread-local instance.
	#[inline(always)]
	pub fn new_thread_local<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator>(global_allocator: &GTACSA, underlying_memory_source: MS) -> Self
	{
		Self(global_allocator.callback_with_thread_local_allocator(|| Rc::new(underlying_memory_source)))
	}

	/// Creates a new coroutine-local instance.
	#[inline(always)]
	pub fn new_coroutine_local<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator>(global_allocator: &GTACSA, underlying_memory_source: MS) -> Self
	{
		Self(global_allocator.callback_with_coroutine_local_allocator(|| Rc::new(underlying_memory_source)))
	}
}
