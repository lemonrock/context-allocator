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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BumpAllocator
{
	most_recent_allocation_pointer: MemoryAddress,
	next_allocation_at_pointer: MemoryAddress,
	ends_at_pointer: MemoryAddress,
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
					return Err(AllocErr)
				}
				unsafe { transmute(pointer) }
			};

			if unlikely!(allocation_ends_at_pointer > $self.ends_at_pointer)
			{
				return Err(AllocErr)
			}

			allocation_ends_at_pointer
		}
	}
}

impl Allocator for BumpAllocator
{
	#[inline(always)]
	fn allocate(&mut self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		debug_assert!(non_zero_power_of_two_alignment <= Self::MaximumPowerOfTwoAlignment, "non_zero_power_of_two_alignment `{}` exceeds `{}`", non_zero_power_of_two_alignment, Self::MaximumPowerOfTwoAlignment);

		let next_allocation_at_rounded_up_pointer = self.next_allocation_at_pointer.round_up_to_power_of_two(non_zero_power_of_two_alignment);

		self.most_recent_allocation_pointer = next_allocation_at_rounded_up_pointer;
		self.next_allocation_at_pointer = allocation_ends_at_pointer!(self, non_zero_size, next_allocation_at_rounded_up_pointer);

		Ok(next_allocation_at_rounded_up_pointer)
	}

	#[inline(always)]
	fn deallocate(&mut self, _non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		if unlikely!(current_memory == self.most_recent_allocation_pointer)
		{
			self.next_allocation_at_pointer = self.most_recent_allocation_pointer
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&mut self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		if unlikely!(current_memory == self.most_recent_allocation_pointer)
		{
			let size = non_zero_new_size.get();
			self.next_allocation_at_pointer = current_memory.add(size)
		}

		Ok(current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&mut self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		if unlikely!(current_memory == self.most_recent_allocation_pointer)
		{
			self.next_allocation_at_pointer = allocation_ends_at_pointer!(self, non_zero_new_size, current_memory);
			Ok(current_memory)
		}
		else
		{
			let result = self.allocate(non_zero_new_size, non_zero_power_of_two_alignment);
			let pointer: *mut u8 = unsafe { transmute(result) };
			if unlikely!(pointer.is_null())
			{
				Err(AllocErr)
			}
			else
			{
				let current_size = non_zero_current_size.get();
				unsafe { pointer.copy_from(current_memory.as_ptr(), current_size) };
				Ok(unsafe { transmute(pointer) })
			}
		}
	}
}

impl BumpAllocator
{
	const MaximumPowerOfTwoAlignment: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(4096) };

	/// New instance wrapping a block of memory.
	#[inline(always)]
	pub fn new(starts_at: MemoryAddress, non_zero_size: NonZeroUsize) -> Self
	{
		Self
		{
			most_recent_allocation_pointer: starts_at,
			next_allocation_at_pointer: starts_at,
			ends_at_pointer: starts_at.add(non_zero_size.get()),
		}
	}
}
