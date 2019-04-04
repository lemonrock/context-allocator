// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// State of an allocator.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AllocatorState
{
	allocator_instance: *const (),
	memory_range: MemoryRange,
	name: &'static str,
}

impl AllocatorState
{
	#[inline(always)]
	const fn new(name: &'static str) -> Self
	{
		Self
		{
			allocator_instance: null(),
			memory_range: MemoryRange::NotInUse,
			name,
		}
	}

	#[inline(always)]
	fn contains(&self, current_memory: MemoryAddress) -> bool
	{
		self.memory_range.contains(current_memory)
	}

	#[inline(always)]
	fn allocate<A: Allocator>(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<NonNull<u8>, AllocErr>
	{
		self.memory_range.debug_assert_is_in_use(self.name);

		self.allocator::<A>().allocate(non_zero_size, non_zero_power_of_two_alignment)
	}

	#[inline(always)]
	fn deallocate<A: Allocator>(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.memory_range.debug_assert_memory_range_contains_end(current_memory, non_zero_size, self.name);

		self.allocator::<A>().deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate<A: Allocator>(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<NonNull<u8>, AllocErr>
	{
		self.memory_range.debug_assert_memory_range_contains_end(current_memory, non_zero_current_size, self.name);

		self.allocator::<A>().growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate<A: Allocator>(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<NonNull<u8>, AllocErr>
	{
		self.memory_range.debug_assert_memory_range_contains_end(current_memory, non_zero_current_size, self.name);

		self.allocator::<A>().shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn allocator<'a, A: 'a + Allocator>(&self) -> &'a A
	{
		debug_assert!(!self.allocator_instance.is_null(), "{} allocator is null", self.name);

		unsafe { & * (self.allocator_instance as *const A) }
	}

	#[inline(always)]
	fn save_coroutine_local_allocator() -> AllocatorState
	{
		Self::coroutine_local_allocator_state().clone()
	}

	#[inline(always)]
	fn restore_coroutine_local_allocator(allocator_state: AllocatorState)
	{
		unsafe { coroutine_local_allocator_state = allocator_state }
	}

	#[inline(always)]
	fn coroutine_local_allocator_state<'a>() -> &'a Self
	{
		unsafe { &coroutine_local_allocator_state }
	}

	#[inline(always)]
	fn thread_local_allocator_state<'a>() -> &'a Self
	{
		unsafe { &thread_local_allocator_state }
	}
}
