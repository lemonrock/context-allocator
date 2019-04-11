// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts an `Allocator` to the `GlobalAlloc` and `Alloc` traits.
#[repr(transparent)]
#[derive(Debug)]
pub struct AllocatorAdaptor<'a, A: 'a + Allocator>(&'a A);

impl<'a, A: 'a + Allocator> Deref for AllocatorAdaptor<'a, A>
{
	type Target = A;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

unsafe impl<'a, A: 'a + Allocator> GlobalAlloc for AllocatorAdaptor<'a, A>
{
	global_alloc!();
}

unsafe impl<'a, A: 'a + Allocator> Alloc for AllocatorAdaptor<'a, A>
{
	alloc!();
}

impl<'a, A: 'a + Allocator> Allocator for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		self.0.allocate(non_zero_size, non_zero_power_of_two_alignment)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.0.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}
}
