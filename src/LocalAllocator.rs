// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A local allocator is an allocator with a known range of memory addresses it uses for allocated memory.
///
/// This allows logic to determine which allocator should be used to free (deallocate) which memory pointers.
///
/// ***It is important that a `LocalAllocator` does nothing on `drop()`***.
pub trait LocalAllocator<MS: MemorySource>: Allocator + Sized + Debug
{
	/// Creates a new instance.
	fn new_local_allocator(memory_source: MS, lifetime_hint: LifetimeHint, block_size_hint: NonZeroUsize) -> Self;
	
	/// The range of memory addresses that can be used to allocate memory by this allocator.
	///
	/// This function is called repeatedly, so ideally should be inline and fast.
	fn memory_range(&self) -> MemoryRange;

	/// Returns `true` if this allocator is responsible for an allocation starting with the given `from_memory_address`.
	///
	/// This function is called repeatedly, so ideally should be inline and fast.
	#[inline(always)]
	fn contains(&self, from_memory_address: MemoryAddress) -> bool
	{
		self.memory_range().contains(from_memory_address)
	}
}
