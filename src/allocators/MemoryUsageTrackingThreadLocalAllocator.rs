// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// An allocator designed to track memory usage.
///
/// This allocator tracks memory usage based on requested memory sizes, not actualy allocated sizes.
/// This is because growing (or shrinking) reallocations do not know the original actually allocated sizes.
///
/// This allocator is not thread-safe.
#[derive(Debug)]
pub struct MemoryUsageTrackingThreadLocalAllocator<LA: LocalAllocator<MemoryMapSource>>
{
	local_allocator: LA,
	
	local_allocator_memory_usage: LocalAllocatorMemoryUsage,
}

impl<LA: LocalAllocator<MemoryMapSource>> Allocator for MemoryUsageTrackingThreadLocalAllocator<LA>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		let result = self.local_allocator.allocate(non_zero_size, non_zero_power_of_two_alignment)?;
		self.local_allocator_memory_usage.allocated(result.1);
		Ok(result)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		self.local_allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory);
		self.local_allocator_memory_usage.deallocated(non_zero_size);
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		let result = self.local_allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved)?;
		self.local_allocator_memory_usage.growing_reallocated(non_zero_current_size, result.1);
		Ok(result)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		let result = self.local_allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved)?;
		self.local_allocator_memory_usage.shrinking_reallocated(non_zero_current_size, result.1);
		Ok(result)
	}
}

impl<LA: LocalAllocator<MemoryMapSource>> LocalAllocator<MemoryMapSource> for MemoryUsageTrackingThreadLocalAllocator<LA>
{
	#[inline(always)]
	fn new_local_allocator(memory_source: MemoryMapSource, lifetime_hint: LifetimeHint, block_size_hint: NonZeroUsize) -> Self
	{
		Self
		{
			local_allocator: LA::new_local_allocator(memory_source, lifetime_hint, block_size_hint),
			local_allocator_memory_usage: Default::default()
		}
	}
	
	#[inline(always)]
	fn memory_range(&self) -> MemoryRange
	{
		self.local_allocator.memory_range()
	}
}

impl<LA: LocalAllocator<MemoryMapSource>> MemoryUsageTrackingThreadLocalAllocator<LA>
{
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(local_allocator: LA) -> Self
	{
		Self
		{
			local_allocator,
			local_allocator_memory_usage: LocalAllocatorMemoryUsage::new(),
		}
	}
	
	/// Memory usage.
	#[inline(always)]
	pub fn memory_usage(&self) -> &LocalAllocatorMemoryUsage
	{
		&self.local_allocator_memory_usage
	}
}
