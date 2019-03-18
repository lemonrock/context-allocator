// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// This is a very simple bump allocator of minimal utility.
///
/// It:-
///
/// * Can efficiently shrink and grow (reallocate) for the most recent allocation made (useful when pushing to a RawVec, say).
/// * Has no wrapping around at the end (but this could be achieved using a mirror ring buffer).
/// * Has no ability to resize in place if dead space occurs before next allocation because of alignment.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BumpAllocator
{
	most_recent_allocation_pointer: usize,
	next_allocation_at_pointer: usize,
	ends_at_pointer: usize,
}

impl Allocator for BumpAllocator
{
	#[inline(always)]
	fn allocate(&mut self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<NonNull<u8>, AllocErr>
	{
		let size = non_zero_size.get();
		let power_of_two_alignment = non_zero_power_of_two_alignment.get();
		debug_assert!(power_of_two_alignment <= Self::MaximumPowerOfTwoAlignment, "non_zero_power_of_two_alignment `{}` exceed `{}`", non_zero_power_of_two_alignment, Self::MaximumPowerOfTwoAlignment);

		debug_assert!(self.next_allocation_at_pointer.checked_add(power_of_two_alignment - 1).is_some(), "next_allocation_at_pointer is far too close to the maximum value of a pointer");
		let next_allocation_at_rounded_up_pointer = (self.next_allocation_at_pointer + power_of_two_alignment - 1) & !(power_of_two_alignment - 1);

		let ends_at_pointer = match next_allocation_at_rounded_up_pointer.checked_add(size)
		{
			None => return Err(AllocErr),
			Some(ends_at_pointer) => ends_at_pointer
		};

		if unlikely!(ends_at_pointer > self.ends_at_pointer)
		{
			return Err(AllocErr)
		}

		self.most_recent_allocation_pointer = next_allocation_at_rounded_up_pointer;
		self.next_allocation_at_pointer = ends_at_pointer;

		Ok(unsafe { NonNull::new_unchecked(next_allocation_at_rounded_up_pointer as *mut u8) })
	}

	#[inline(always)]
	fn deallocate(&mut self, _non_zero_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, current_memory: NonNull<u8>)
	{
		let current_memory_pointer = current_memory.as_ptr() as usize;
		if unlikely!(current_memory_pointer == self.most_recent_allocation_pointer)
		{
			self.next_allocation_at_pointer = self.most_recent_allocation_pointer
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&mut self, non_zero_new_size: NonZeroUsize, _non_zero_power_of_two_alignment: NonZeroUsize, _non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		let current_memory_pointer = current_memory.as_ptr() as usize;
		if unlikely!(current_memory_pointer == self.most_recent_allocation_pointer)
		{
			let size = non_zero_new_size.get();
			self.next_allocation_at_pointer = current_memory_pointer + size
		}

		Ok(current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&mut self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: NonNull<u8>) -> Result<NonNull<u8>, AllocErr>
	{
		let current_memory_pointer = current_memory.as_ptr() as usize;
		if unlikely!(current_memory_pointer == self.most_recent_allocation_pointer)
		{
			let new_size = non_zero_new_size.get();

			let ends_at_pointer = match current_memory_pointer.checked_add(new_size)
			{
				None => return Err(AllocErr),
				Some(ends_at_pointer) => ends_at_pointer
			};

			if unlikely!(ends_at_pointer > self.ends_at_pointer)
			{
				return Err(AllocErr)
			}

			self.next_allocation_at_pointer = current_memory_pointer + new_size;
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
	const MaximumPowerOfTwoAlignment: usize = 4096;

	/// New instance wrapping a block of memory.
	#[inline(always)]
	pub fn new(starts_at: NonNull<u8>, non_zero_size: NonZeroUsize) -> Self
	{
		let starts_at = starts_at.as_ptr() as usize;

		Self
		{
			most_recent_allocation_pointer: starts_at,
			next_allocation_at_pointer: starts_at,
			ends_at_pointer: starts_at + non_zero_size.get(),
		}
	}
}
