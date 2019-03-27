// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.


/// A helper trait that brings together the core, common functionality required to implement the traits `GlobalAlloc` and `Alloc`.
pub trait Allocator: Debug + Sized
{
	/// The sentinel value used for a zero-sized allocation.
	const ZeroSizedAllocation: MemoryAddress = non_null_pointer(::std::usize::MAX as *mut u8);

	/// Allocate memory.
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>;

	/// Deallocate (free) memory.
	///
	/// The parameter `memory` will never be the value `Self::ZeroSizedAllocation` and will always have been allocated by this `Allocator`.
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress);

	/// Reallocate memory by growing it.
	///
	/// `non_zero_new_size` will always be greater than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Reallocate memory by shrinking it.
	///
	/// `non_zero_new_size` will always be less than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Adapts to a `GlobalAlloc` and `Alloc`.
	#[inline(always)]
	fn adapt<'a>(&'a self) -> AllocatorAdaptor<'a, Self>
	{
		AllocatorAdaptor(self)
	}

	/// Adapts a reference to a `GlobalAlloc` and `Alloc` reference.
	#[inline(always)]
	fn adapt_reference<'a>(&'a self) -> &'a AllocatorAdaptor<'a, Self>
	{
		unsafe { transmute(self) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn allocate_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Ok(Self::ZeroSizedAllocation)
		}

		let non_zero_size = layout.size_.non_zero();
		let result = self.allocate(non_zero_size, layout.align_);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer = unsafe { transmute::<_, *mut u8>(result) };

		if likely!(!pointer.is_null())
		{
			unsafe { pointer.write_bytes(0x00, zero_size) };
		}

		unsafe { transmute(pointer) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn reallocate(&self, current_memory: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let current_size = layout.size_;

		if unlikely!(current_size == new_size)
		{
			return Ok(current_memory)
		}

		let non_zero_power_of_two_alignment = layout.align_;

		if likely!(new_size > current_size)
		{
			let non_zero_new_size = new_size.non_zero();

			if unlikely!(current_size == 0)
			{
				return self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)
			}

			let non_zero_current_size = current_size.non_zero();
			self.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
		else
		{
			let non_zero_current_size = current_size.non_zero();

			if unlikely!(new_size == 0)
			{
				self.deallocate(non_zero_current_size, non_zero_power_of_two_alignment, current_memory);
				return Ok(Self::ZeroSizedAllocation)
			}

			let non_zero_new_size = new_size.non_zero();
			self.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
	}
}
