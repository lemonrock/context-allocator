// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This is a very simple bump allocator of minimal utility.
///
/// It:-
///
/// * Can efficiently shrink and grow (reallocate) for the most recent allocation made (useful when pushing to a RawVec, say).
/// * Has no wrapping around at the end (but this could be achieved using a mirror ring buffer).
/// * Has no ability to resize in place if dead space occurs before next allocation because of alignment.
///
/// Is suitable for use with short-lived coroutines, such as those used to make a DNS query.
///
/// This allocator NEVER grows or shrinks its memory region.
///
/// This allocator is not thread-safe.
#[derive(Debug)]
pub struct BumpAllocator<MS: MemorySource>
{
	most_recent_allocation_pointer: Cell<MemoryAddress>,
	next_allocation_at_pointer: Cell<MemoryAddress>,
	ends_at_pointer: MemoryAddress,

	memory_source: MS,
}

macro_rules! allocation_ends_at_pointer
{
	($self: ident, $non_zero_size: ident, $allocation_from: ident) =>
	{
		{
			// NOTE: This evil code is used so that we can use an if hint of `unlikely!` rather than an unhinted `match` for `result`.
			let allocation_ends_at_pointer: MemoryAddress =
			{
				let size = $non_zero_size.get();
				let pointer: *mut u8 = unsafe { transmute($allocation_from.checked_add(size)) };
				if unlikely!(pointer.is_null())
				{
					return Err(AllocError)
				}
				unsafe { transmute(pointer) }
			};

			if unlikely!(allocation_ends_at_pointer > $self.ends_at_pointer)
			{
				return Err(AllocError)
			}

			allocation_ends_at_pointer
		}
	}
}

impl<MS: MemorySource> Allocator for BumpAllocator<MS>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		debug_assert!(non_zero_power_of_two_alignment <= Self::MaximumPowerOfTwoAlignment, "non_zero_power_of_two_alignment `{}` exceeds `{}`", non_zero_power_of_two_alignment, Self::MaximumPowerOfTwoAlignment);

		let next_allocation_at_rounded_up_pointer = self.next_allocation_at_pointer.get().round_up_to_power_of_two(non_zero_power_of_two_alignment);

		self.most_recent_allocation_pointer.set(next_allocation_at_rounded_up_pointer);
		self.next_allocation_at_pointer.set(allocation_ends_at_pointer!(self, non_zero_size, next_allocation_at_rounded_up_pointer));
		let actual_size = (self.next_allocation_at_pointer.get().as_ptr() as usize) - (next_allocation_at_rounded_up_pointer.as_ptr() as usize);

		Ok((next_allocation_at_rounded_up_pointer, actual_size))
	}

	#[inline(always)]
	fn deallocate(&self, _non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		if unlikely!(current_memory == self.most_recent_allocation_pointer.get())
		{
			self.next_allocation_at_pointer.set(self.most_recent_allocation_pointer.get())
		}
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		debug_assert!(non_zero_power_of_two_new_alignment <= Self::MaximumPowerOfTwoAlignment, "non_zero_power_of_two_new_alignment `{}` exceeds `{}`", non_zero_power_of_two_new_alignment, Self::MaximumPowerOfTwoAlignment);
		
		if unlikely!(self.fits_at_current_location(non_zero_power_of_two_new_alignment, current_memory))
		{
			let last = self.most_recent_allocation_pointer.get();
			self.next_allocation_at_pointer.set(allocation_ends_at_pointer!(self, non_zero_new_size, current_memory));

			let actual_size = (self.next_allocation_at_pointer.get().as_ptr() as usize) - (last.as_ptr() as usize);
			Ok((current_memory, actual_size))
		}
		else
		{
			self.allocate_and_copy(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved, non_zero_current_size.get())
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool) -> Result<(NonNull<u8>, usize), AllocError>
	{
		debug_assert!(non_zero_power_of_two_new_alignment <= Self::MaximumPowerOfTwoAlignment, "non_zero_power_of_two_new_alignment `{}` exceeds `{}`", non_zero_power_of_two_new_alignment, Self::MaximumPowerOfTwoAlignment);
		
		let new_size = non_zero_new_size.get();
		if unlikely!(self.fits_at_current_location(non_zero_power_of_two_new_alignment, current_memory))
		{
			self.next_allocation_at_pointer.set(current_memory.add(new_size));
			Ok((current_memory, new_size))
		}
		else if likely!(Self::new_alignment_can_be_accommodated(non_zero_power_of_two_new_alignment, current_memory))
		{
			Ok((current_memory, new_size))
		}
		else
		{
			self.allocate_and_copy(non_zero_new_size, non_zero_power_of_two_new_alignment, non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory, current_memory_can_not_be_moved, non_zero_new_size.get())
		}
	}
}

impl<MS: MemorySource> LocalAllocator<MS> for BumpAllocator<MS>
{
	#[inline(always)]
	fn new_local_allocator(memory_source: MS, _lifetime_hint: LifetimeHint, _block_size_hint: NonZeroUsize) -> Self
	{
		Self::new(memory_source)
	}
	
	#[inline(always)]
	fn memory_range(&self) -> MemoryRange
	{
		MemoryRange::new(self.allocations_start_from(), self.ends_at_pointer)
	}
}

impl<MS: MemorySource> BumpAllocator<MS>
{
	const MaximumPowerOfTwoAlignment: NonZeroUsize = non_zero_usize(4096);

	/// New instance wrapping a block of memory.
	#[inline(always)]
	pub fn new(memory_source: MS) -> Self
	{
		let allocations_start_from = memory_source.allocations_start_from();

		Self
		{
			most_recent_allocation_pointer: Cell::new(allocations_start_from),
			next_allocation_at_pointer: Cell::new(allocations_start_from),
			ends_at_pointer: allocations_start_from.add_non_zero(memory_source.size()),

			memory_source,
		}
	}

	#[inline(always)]
	fn allocations_start_from(&self) -> MemoryAddress
	{
		self.ends_at_pointer.subtract_non_zero(self.memory_source.size())
	}
	
	#[inline(always)]
	fn fits_at_current_location(&self, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>) -> bool
	{
		current_memory == self.most_recent_allocation_pointer.get() && Self::new_alignment_can_be_accommodated(non_zero_power_of_two_current_alignment, current_memory)
	}
	
	#[inline(always)]
	fn new_alignment_can_be_accommodated(non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>) -> bool
	{
		current_memory.is_aligned_to(non_zero_power_of_two_current_alignment)
	}
	
	#[inline(always)]
	fn allocate_and_copy(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_new_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, non_zero_power_of_two_current_alignment: NonZeroUsize, current_memory: NonNull<u8>, current_memory_can_not_be_moved: bool, amount_to_copy: usize) -> Result<(NonNull<u8>, usize), AllocError>
	{
		if unlikely!(current_memory_can_not_be_moved)
		{
			return Err(AllocError)
		}
		
		let (new_memory, actual_size) = self.allocate(non_zero_new_size, non_zero_power_of_two_new_alignment)?;
		unsafe { new_memory.as_ptr().copy_from(current_memory.as_ptr(), amount_to_copy) };
		self.deallocate(non_zero_current_size, non_zero_power_of_two_current_alignment, current_memory);
		Ok((new_memory, actual_size))
	}
}
