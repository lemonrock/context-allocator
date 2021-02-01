// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// Adapts an `Allocator` to the `GlobalAlloc` and `Alloc` traits.
#[derive(Debug)]
#[repr(transparent)]
pub struct AllocatorAdaptor<'a, A: 'a + Allocator + ?Sized>(pub(crate) &'a A);

impl<'a, A: 'a + Allocator> Deref for AllocatorAdaptor<'a, A>
{
	type Target = A;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

unsafe impl<'a, A: 'a + Allocator + ?Sized> GlobalAlloc for AllocatorAdaptor<'a, A>
{
	global_alloc!();
}

unsafe impl<'a, A: 'a + Allocator + ?Sized> AllocRef for AllocatorAdaptor<'a, A>
{
	alloc_ref!();
}

impl<'a, A: 'a + Allocator + ?Sized> Allocator for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		self.0.allocate(non_zero_size, non_zero_power_of_two_alignment)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		self.0.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		self.0.growing_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		self.0.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved)
	}
}
